use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
use crate::utils::command::run_cmd_first_line;
use std::fs;
use std::path::PathBuf;

pub struct ShellChecker;

impl Checker for ShellChecker {
    fn id(&self) -> &'static str {
        "shell"
    }

    fn title(&self) -> &'static str {
        "Shell & Configuration"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::System
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["bash", "zsh", "fish"]
    }

    fn is_installed(&self) -> bool {
        true
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Current Shell detection
        let shell_env = std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());
        let mut shell_item = DiagnosticItem::ok("Login Shell");
        shell_item.path = Some(shell_env.clone());

        if let Some(shell_bin) = shell_env.split('/').last() {
            if let Some(v) = run_cmd_first_line(shell_bin, &["--version"]) {
                shell_item.version = Some(v);
            }
        }
        result.items.push(shell_item);

        // 2. Shell Configuration Files Inspection (~/.bashrc, ~/.zshrc, ~/.profile)
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            let home_path = PathBuf::from(home);
            let config_files = [".bashrc", ".zshrc", ".profile", ".bash_profile"];
            let mut detected_configs = Vec::new();

            for filename in config_files {
                let file_path = home_path.join(filename);
                if file_path.exists() {
                    let mut rc_item = DiagnosticItem::ok(format!("Configuration: ~/{filename}"));
                    rc_item.path = Some(file_path.to_string_lossy().to_string());

                    if let Ok(content) = fs::read_to_string(&file_path) {
                        let lines: Vec<&str> = content.lines().collect();
                        rc_item.details.push(format!("Total lines: {}", lines.len()));

                        // Analyze for duplicate export PATH entries
                        let path_exports: Vec<&str> = lines
                            .iter()
                            .filter(|l| {
                                let trimmed = l.trim();
                                !trimmed.starts_with('#') && trimmed.contains("export PATH=")
                            })
                            .copied()
                            .collect();

                        if path_exports.len() > 3 {
                            rc_item.status = Status::Info;
                            rc_item.issues.push(Issue::new(
                                Status::Info,
                                format!("Found {} 'export PATH=' entries. Consider consolidating them.", path_exports.len()),
                            ));
                        }

                        // Check for common syntax anti-patterns (e.g. export PATH=$PATH without colon)
                        for line in &lines {
                            let t = line.trim();
                            if t.starts_with("export PATH=") && !t.contains(':') && !t.contains('$') {
                                rc_item.status = Status::Warning;
                                let mut issue = Issue::with_detail(
                                    Status::Warning,
                                    "Potential destructive PATH overwrite detected (missing existing $PATH reference)",
                                    t.to_string(),
                                );
                                issue.cause = Some("Configuration line assigns PATH without expanding existing $PATH".into());
                                issue.impact = Some("System binaries (/usr/bin, /bin) become inaccessible, breaking basic terminal commands".into());
                                rc_item.issues.push(issue);

                                let rec = Recommendation::full(
                                    "Ensure PATH exports prepend or append to $PATH",
                                    "export PATH=\"/your/new/path:$PATH\"",
                                    "Overwriting PATH completely without :$PATH breaks standard system commands.",
                                )
                                .with_verification("echo $PATH");
                                rc_item.recommendations.push(rec);
                            }
                        }
                    }
                    detected_configs.push(rc_item);
                }
            }

            if detected_configs.is_empty() {
                let mut no_cfg = DiagnosticItem::info("Shell Configuration Files");
                no_cfg.details.push("No common shell config files (.bashrc, .zshrc) found in home directory".to_string());
                result.items.push(no_cfg);
            } else {
                result.items.extend(detected_configs);
            }
        }

        result
    }
}
