use crate::core::{CategoryResult, FullDiagnosticReport, Status};
use colored::*;

pub struct TerminalFormatter {
    pub verbose: bool,
    pub quiet: bool,
}

impl TerminalFormatter {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub fn print_banner(&self) {
        if self.quiet {
            return;
        }
        println!();
        println!(
            "{}",
            " 🩺  envdoctor  -  Developer Environment Diagnostic Tool"
                .bold()
                .cyan()
        );
        println!("{}", " ────────────────────────────────────────────────────────────".bright_black());
    }

    pub fn print_report(&self, report: &FullDiagnosticReport) {
        self.print_banner();

        if !self.quiet {
            println!(
                " {} OS: {} {} | Arch: {} | Kernel: {}",
                "ℹ".blue().bold(),
                report.system.os.bold(),
                report.system.os_version,
                report.system.arch,
                report.system.kernel_version
            );
            println!("{}", " ────────────────────────────────────────────────────────────".bright_black());
            println!();
        }

        for category in &report.results {
            self.print_category(category);
        }

        self.print_summary(report);
    }

    pub fn print_category(&self, cat: &CategoryResult) {
        let cat_status = cat.overall_status();
        let status_badge = match cat_status {
            Status::Ok => "  OK  ".on_green().black().bold(),
            Status::Info => " INFO ".on_blue().white().bold(),
            Status::Warning => " WARN ".on_yellow().black().bold(),
            Status::Error => " FAIL ".on_red().white().bold(),
            Status::Critical => " CRIT ".on_magenta().white().bold(),
        };

        if self.quiet && cat_status.is_ok() {
            return;
        }

        println!("{} {}", status_badge, cat.title.bold().underline());

        for item in &cat.items {
            if self.quiet && item.status.is_ok() {
                continue;
            }

            let (symbol, sym_color) = match item.status {
                Status::Ok => ("✔", Color::Green),
                Status::Info => ("ℹ", Color::Blue),
                Status::Warning => ("▲", Color::Yellow),
                Status::Error => ("✖", Color::Red),
                Status::Critical => ("🔥", Color::Magenta),
            };

            let mut header = format!("   {} {}", symbol.color(sym_color).bold(), item.name.bold());
            if let Some(ref ver) = item.version {
                header.push_str(&format!(" ({})", ver.bright_cyan()));
            }
            println!("{}", header);

            if let Some(ref path) = item.path {
                if self.verbose || !item.status.is_ok() {
                    println!("     {} {}", "Path:".bright_black(), path.bright_black());
                }
            }

            if self.verbose || !item.status.is_ok() {
                for detail in &item.details {
                    println!("     {} {}", "•".bright_black(), detail);
                }
            }

            for issue in &item.issues {
                let issue_prefix = match issue.status {
                    Status::Warning => "Warning:".yellow().bold(),
                    Status::Error => "Error:".red().bold(),
                    Status::Critical => "Critical:".magenta().bold(),
                    _ => "Note:".blue().bold(),
                };
                println!("     {} {}", issue_prefix, issue.message);
                if let Some(ref d) = issue.detail {
                    println!("       {} {}", "Detail:".bright_black(), d.bright_black());
                }
                if let Some(ref cause) = issue.cause {
                    println!("       {} {}", "Cause:".bright_black().bold(), cause.bright_black());
                }
                if let Some(ref impact) = issue.impact {
                    println!("       {} {}", "Impact:".bright_yellow().bold(), impact);
                }
            }

            for rec in &item.recommendations {
                println!("     {} {}", "💡 Recommendation:".bright_yellow().bold(), rec.action);
                if let Some(ref cmd) = rec.command {
                    println!("        {} {}", "$".bright_green(), cmd.bright_white().bold());
                }
                if let Some(ref exp) = rec.explanation {
                    println!("        {}", exp.bright_black());
                }
                if let Some(ref verif) = rec.verification {
                    println!("        {} {}", "Verify: $".bright_cyan(), verif.bright_cyan());
                }
            }
        }
        println!();
    }

    pub fn print_summary(&self, report: &FullDiagnosticReport) {
        let summary = &report.summary;
        let issues = report.collect_issues();

        println!("{}", " ────────────────────────────────────────────────────────────".bright_black());
        println!(" {}", "Summary".bold().underline());
        if summary.ok > 0 {
            println!("   {} {}", "✔".green().bold(), format!("{} checks passed", summary.ok).green());
        }
        if summary.warning > 0 {
            println!("   {} {}", "▲".yellow().bold(), format!("{} warnings", summary.warning).yellow());
        }
        if summary.error > 0 {
            println!("   {} {}", "✖".red().bold(), format!("{} errors", summary.error).red());
        }
        if summary.critical > 0 {
            println!("   {} {}", "🔥".magenta().bold(), format!("{} critical issues", summary.critical).magenta());
        }
        if summary.info > 0 {
            println!("   {} {}", "ℹ".blue().bold(), format!("{} informational", summary.info).blue());
        }

        if !issues.is_empty() {
            println!("\n {}", "Issues:".bold().yellow());
            for (idx, issue) in issues.iter().enumerate() {
                let badge = match issue.status {
                    Status::Critical => "[CRITICAL]".magenta().bold(),
                    Status::Error => "[ERROR]".red().bold(),
                    Status::Warning => "[WARN]".yellow().bold(),
                    _ => "[INFO]".blue().bold(),
                };
                println!("   {}. {} {}: {}", idx + 1, badge, issue.category.bold(), issue.message);
                if let Some(ref impact) = issue.impact {
                    println!("      {} {}", "Impact:".bright_yellow(), impact.bright_black());
                }
                if let Some(ref fix) = issue.fix_command {
                    println!("      {} {}", "Fix:".bright_green(), format!("$ {}", fix).bright_white());
                }
            }
            println!("\n {}", "Run `envdr fix` to preview or apply available fixes.".bright_cyan());
            println!(" {}", "Run `envdr --help` for more options.".bright_black());
        } else {
            println!("\n {}", "✨ All checks passed! Your environment looks healthy.".bright_green().bold());
        }
        println!();
    }
}
