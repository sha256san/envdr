use crate::core::{FullDiagnosticReport, Status};

pub struct MarkdownFormatter;

impl MarkdownFormatter {
    pub fn format(report: &FullDiagnosticReport) -> String {
        let mut md = String::new();

        md.push_str("# 🩺 envdoctor Diagnostic Report\n\n");
        md.push_str(&format!("- **Timestamp**: `{}`\n", report.timestamp));
        md.push_str(&format!("- **OS**: {} {} ({})\n", report.system.os, report.system.os_version, report.system.arch));
        md.push_str(&format!("- **Kernel**: `{}`\n", report.system.kernel_version));
        md.push_str(&format!("- **CPU**: {} ({} cores)\n", report.system.cpu_brand, report.system.cpu_count));
        md.push_str(&format!("- **Memory**: {} MB / {} MB used\n\n", report.system.used_memory_mb, report.system.total_memory_mb));

        md.push_str("## 📊 Summary\n\n");
        md.push_str("| Status | Count |\n");
        md.push_str("| :--- | :--- |\n");
        md.push_str(&format!("| ✔ OK | {} |\n", report.summary.ok));
        md.push_str(&format!("| ℹ INFO | {} |\n", report.summary.info));
        md.push_str(&format!("| ⚠ WARNING | {} |\n", report.summary.warning));
        md.push_str(&format!("| ✖ ERROR | {} |\n", report.summary.error));
        md.push_str(&format!("| 🔥 CRITICAL | {} |\n\n", report.summary.critical));

        md.push_str("## 🔍 Detailed Results\n\n");

        for cat in &report.results {
            let cat_badge = match cat.overall_status() {
                Status::Ok => "🟢 OK",
                Status::Info => "🔵 INFO",
                Status::Warning => "🟡 WARNING",
                Status::Error => "🔴 ERROR",
                Status::Critical => "🟣 CRITICAL",
            };
            md.push_str(&format!("### {} {}\n\n", cat_badge, cat.title));

            for item in &cat.items {
                let item_icon = item.status.symbol();
                let ver_str = item.version.as_deref().map(|v| format!(" (`{}`)", v)).unwrap_or_default();
                md.push_str(&format!("- **{} {}**{}\n", item_icon, item.name, ver_str));

                if let Some(ref path) = item.path {
                    md.push_str(&format!("  - Path: `{}`\n", path));
                }

                for detail in &item.details {
                    md.push_str(&format!("  - {}\n", detail));
                }

                for issue in &item.issues {
                    md.push_str(&format!("  - ⚠️ **Problem**: {}\n", issue.message));
                    if let Some(ref d) = issue.detail {
                        md.push_str(&format!("    - Detail: `{}`\n", d));
                    }
                }

                for rec in &item.recommendations {
                    md.push_str(&format!("  - 💡 **Recommendation**: {}\n", rec.action));
                    if let Some(ref cmd) = rec.command {
                        md.push_str(&format!("    ```bash\n    {}\n    ```\n", cmd));
                    }
                    if let Some(ref exp) = rec.explanation {
                        md.push_str(&format!("    > {}\n", exp));
                    }
                }
            }
            md.push('\n');
        }

        md
    }
}
