use crate::core::{CategoryResult, Status};
use colored::*;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct FixAction {
    pub category: String,
    pub title: String,
    pub description: String,
    pub command: String,
    pub requires_sudo: bool,
}

pub struct AutoFixer;

impl AutoFixer {
    /// 診断結果から修復可能なアクションを抽出
    pub fn plan_fixes(results: &[CategoryResult], target_filter: Option<&str>) -> Vec<FixAction> {
        let mut fixes = Vec::new();

        for cat in results {
            if let Some(target) = target_filter {
                if !cat.category.eq_ignore_ascii_case(target) {
                    continue;
                }
            }

            for item in &cat.items {
                if item.status == Status::Ok {
                    continue;
                }

                for rec in &item.recommendations {
                    if let Some(ref cmd) = rec.command {
                        // URLだけのものはコマンドではないので除外
                        if cmd.starts_with("http://") || cmd.starts_with("https://") {
                            continue;
                        }

                        let requires_sudo = cmd.contains("sudo ");
                        fixes.push(FixAction {
                            category: cat.category.clone(),
                            title: format!("Fix: {}", item.name),
                            description: rec.action.clone(),
                            command: cmd.clone(),
                            requires_sudo,
                        });
                    }
                }
            }
        }

        fixes
    }

    /// シェル設定ファイルのバックアップを作成
    pub fn backup_shell_configs() {
        if let Some(home) = std::env::var_os("HOME") {
            let home_path = std::path::PathBuf::from(home);
            for cfg in &[".bashrc", ".zshrc", ".profile"] {
                let src = home_path.join(cfg);
                if src.exists() {
                    let bak = home_path.join(format!("{}.envdr.bak", cfg));
                    let _ = std::fs::copy(&src, &bak);
                }
            }
        }
    }

    /// 修復プランを表示（dry-run）または適用
    pub fn execute_plan(fixes: &[FixAction], apply: bool) -> anyhow::Result<()> {
        if fixes.is_empty() {
            println!("{}", "✨ No auto-fixable issues detected!".green().bold());
            return Ok(());
        }

        println!();
        println!(
            "{}",
            format!(" 🛠️  envdoctor fix - Planned Actions (Total: {})", fixes.len())
                .bold()
                .cyan()
        );
        println!("{}", " ────────────────────────────────────────────────────────────".bright_black());

        for (idx, fix) in fixes.iter().enumerate() {
            println!(
                " [{}] {} ({})",
                (idx + 1).to_string().bold(),
                fix.title.bold(),
                fix.category.bright_blue()
            );
            println!("     {} {}", "Description:".bright_black(), fix.description);
            println!("     {} {}", "Command:".bright_yellow(), fix.command.bright_white().bold());
            if fix.requires_sudo {
                println!("     {}", "⚠️  Requires root / sudo privileges".bright_magenta());
            }
            println!();
        }

        if !apply {
            println!("{}", " ────────────────────────────────────────────────────────────".bright_black());
            println!(
                " {}",
                "ℹ [DRY-RUN MODE] No changes were made.".bright_yellow().bold()
            );
            println!(
                " {}",
                "To apply these fixes automatically, run with: envdoctor --fix --apply (or envdr fix --apply)"
                    .bright_cyan()
            );
            println!();
            return Ok(());
        }

        // Apply mode: create backups first
        Self::backup_shell_configs();
        println!("{}", " ────────────────────────────────────────────────────────────".bright_black());
        println!("{}", " 🚀 Applying fixes (Configuration backup saved to ~/.bashrc.envdr.bak)...".green().bold());
        println!();

        for (idx, fix) in fixes.iter().enumerate() {
            print!(" [{}] Executing: {} ... ", idx + 1, fix.command.bright_white());
            let status = Command::new("sh")
                .arg("-c")
                .arg(&fix.command)
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("{}", "✔ SUCCESS".green().bold());
                }
                Ok(s) => {
                    println!("{}", format!("✖ FAILED (exit code: {:?})", s.code()).red().bold());
                }
                Err(e) => {
                    println!("{}", format!("✖ ERROR: {}", e).red().bold());
                }
            }
        }

        println!();
        println!("{}", "✨ Fix execution completed.".green().bold());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{DiagnosticItem, Recommendation};

    #[test]
    fn test_plan_fixes_extraction() {
        let mut cat = CategoryResult::new("git", "Git");
        let mut item = DiagnosticItem::warning("Git Config", "Git email not set");
        item.recommendations.push(Recommendation::with_command(
            "Set git email",
            "git config --global user.email test@example.com",
        ));
        cat.items.push(item);

        let fixes = AutoFixer::plan_fixes(&[cat], None);
        assert_eq!(fixes.len(), 1);
        assert_eq!(fixes[0].category, "git");
        assert_eq!(fixes[0].command, "git config --global user.email test@example.com");
        assert!(!fixes[0].requires_sudo);
    }

    #[test]
    fn test_plan_fixes_sudo_detection() {
        let mut cat = CategoryResult::new("docker", "Docker");
        let mut item = DiagnosticItem::error("Docker Socket", "Permission denied");
        item.recommendations.push(Recommendation::with_command(
            "Add to docker group",
            "sudo usermod -aG docker $USER",
        ));
        cat.items.push(item);

        let fixes = AutoFixer::plan_fixes(&[cat], None);
        assert_eq!(fixes.len(), 1);
        assert!(fixes[0].requires_sudo);
    }
}

