use crate::core::{FullDiagnosticReport, Status};

pub struct MarkdownFormatter;

impl MarkdownFormatter {
    pub fn format(report: &FullDiagnosticReport) -> String {
        let mut md = String::new();

        md.push_str("# envdoctor Diagnostic Report\n\n");
        md.push_str(&format!("- **Timestamp**: `{}`\n", report.timestamp));
        md.push_str(&format!("- **OS**: {} {} ({})\n", report.system.os, report.system.os_version, report.system.arch));
        md.push_str(&format!("- **Kernel**: `{}`\n", report.system.kernel_version));
        md.push_str(&format!("- **CPU**: {} ({} cores)\n", report.system.cpu_brand, report.system.cpu_count));
        md.push_str(&format!("- **Memory**: {} MB / {} MB used\n\n", report.system.used_memory_mb, report.system.total_memory_mb));

        md.push_str("## Summary\n\n");
        md.push_str("| Status | Count |\n");
        md.push_str("| :--- | :--- |\n");
        md.push_str(&format!("| ✔ Passed | {} |\n", report.summary.ok));
        md.push_str(&format!("| ℹ Info | {} |\n", report.summary.info));
        md.push_str(&format!("| ▲ Warnings | {} |\n", report.summary.warning));
        md.push_str(&format!("| ✖ Errors | {} |\n", report.summary.error));
        md.push_str(&format!("| 🔥 Critical | {} |\n\n", report.summary.critical));

        let issues = report.collect_issues();
        if !issues.is_empty() {
            md.push_str("## Detected Issues\n\n");
            md.push_str("| # | Severity | Category | Issue | Impact | Recommended Fix |\n");
            md.push_str("| :---: | :--- | :--- | :--- | :--- | :--- |\n");
            for (idx, issue) in issues.iter().enumerate() {
                let sev_str = match issue.status {
                    Status::Critical => "🔥 Critical",
                    Status::Error => "✖ Error",
                    Status::Warning => "▲ Warning",
                    _ => "ℹ Info",
                };
                let impact_str = issue.impact.as_deref().unwrap_or("-");
                let fix_str = issue
                    .fix_command
                    .as_deref()
                    .map(|cmd| format!("`{}`", cmd))
                    .unwrap_or_else(|| "-".to_string());

                md.push_str(&format!(
                    "| {} | {} | **{}** | {} | {} | {} |\n",
                    idx + 1,
                    sev_str,
                    issue.category,
                    issue.message,
                    impact_str,
                    fix_str
                ));
            }
            md.push_str("\n");
        }

        md.push_str("## Detailed Results\n\n");

        for cat in &report.results {
            let cat_badge = match cat.overall_status() {
                Status::Ok => "OK",
                Status::Info => "INFO",
                Status::Warning => "WARN",
                Status::Error => "FAIL",
                Status::Critical => "CRITICAL",
            };
            md.push_str(&format!("### [{}] {}\n\n", cat_badge, cat.title));

            for item in &cat.items {
                let item_icon = item.status.symbol();
                let ver_str = item.version.as_deref().map(|v| format!(" (`{}`)", v)).unwrap_or_default();
                md.push_str(&format!("- **{} {}**{}\n", item_icon, item.name, ver_str));

                if let Some(ref path) = item.path {
                    md.push_str(&format!("  - **Path**: `{}`\n", path));
                }

                for detail in &item.details {
                    md.push_str(&format!("  - {}\n", detail));
                }

                for issue in &item.issues {
                    md.push_str(&format!("  - **Problem**: {}\n", issue.message));
                    if let Some(ref d) = issue.detail {
                        md.push_str(&format!("    - **Detail**: `{}`\n", d));
                    }
                    if let Some(ref cause) = issue.cause {
                        md.push_str(&format!("    - **Cause**: {}\n", cause));
                    }
                    if let Some(ref impact) = issue.impact {
                        md.push_str(&format!("    - **Impact**: {}\n", impact));
                    }
                }

                for rec in &item.recommendations {
                    md.push_str(&format!("  - **Recommendation**: {}\n", rec.action));
                    if let Some(ref cmd) = rec.command {
                        md.push_str(&format!("    ```bash\n    {}\n    ```\n", cmd));
                    }
                    if let Some(ref exp) = rec.explanation {
                        md.push_str(&format!("    > {}\n", exp));
                    }
                    if let Some(ref verif) = rec.verification {
                        md.push_str(&format!("    - **Verify**: `{}`\n", verif));
                    }
                }
            }
            md.push('\n');
        }

        md
    }
}
