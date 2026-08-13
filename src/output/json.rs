use crate::core::FullDiagnosticReport;
use anyhow::Result;

pub struct JsonFormatter;

impl JsonFormatter {
    pub fn format(report: &FullDiagnosticReport) -> Result<String> {
        let json_str = serde_json::to_string_pretty(report)?;
        Ok(json_str)
    }
}
