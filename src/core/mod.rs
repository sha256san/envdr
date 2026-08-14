pub mod fixer;

use serde::{Deserialize, Serialize};

/// 診断ステータス
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Ok,
    Info,
    Warning,
    Error,
    Critical,
}

impl Status {
    pub fn is_ok(&self) -> bool {
        matches!(self, Status::Ok | Status::Info)
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Status::Ok => "✔",
            Status::Info => "ℹ",
            Status::Warning => "▲",
            Status::Error => "✖",
            Status::Critical => "🔥",
        }
    }
}

/// 検出された問題点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub status: Status,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
}

impl Issue {
    pub fn new(status: Status, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            detail: None,
            cause: None,
            impact: None,
        }
    }

    pub fn with_detail(status: Status, message: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            detail: Some(detail.into()),
            cause: None,
            impact: None,
        }
    }

    pub fn with_cause(mut self, cause: impl Into<String>) -> Self {
        self.cause = Some(cause.into());
        self
    }

    pub fn with_impact(mut self, impact: impl Into<String>) -> Self {
        self.impact = Some(impact.into());
        self
    }

    pub fn full(
        status: Status,
        message: impl Into<String>,
        detail: Option<String>,
        cause: Option<String>,
        impact: Option<String>,
    ) -> Self {
        Self {
            status,
            message: message.into(),
            detail,
            cause,
            impact,
        }
    }
}

/// 解決策・改善提案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<String>,
}

impl Recommendation {
    pub fn new(action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            command: None,
            explanation: None,
            verification: None,
        }
    }

    pub fn with_command(action: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            command: Some(command.into()),
            explanation: None,
            verification: None,
        }
    }

    pub fn full(action: impl Into<String>, command: impl Into<String>, explanation: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            command: Some(command.into()),
            explanation: Some(explanation.into()),
            verification: None,
        }
    }

    pub fn with_verification(mut self, verification: impl Into<String>) -> Self {
        self.verification = Some(verification.into());
        self
    }

    pub fn complete(
        action: impl Into<String>,
        command: Option<String>,
        explanation: Option<String>,
        verification: Option<String>,
    ) -> Self {
        Self {
            action: action.into(),
            command,
            explanation,
            verification,
        }
    }
}

/// 診断項目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticItem {
    pub name: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<Issue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<Recommendation>,
}

impl DiagnosticItem {
    pub fn ok(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: Status::Ok,
            version: None,
            path: None,
            details: Vec::new(),
            issues: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn info(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: Status::Info,
            version: None,
            path: None,
            details: Vec::new(),
            issues: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn warning(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: Status::Warning,
            version: None,
            path: None,
            details: Vec::new(),
            issues: vec![Issue::new(Status::Warning, message)],
            recommendations: Vec::new(),
        }
    }

    pub fn error(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: Status::Error,
            version: None,
            path: None,
            details: Vec::new(),
            issues: vec![Issue::new(Status::Error, message)],
            recommendations: Vec::new(),
        }
    }
}

/// カテゴリ別診断結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResult {
    pub category: String,
    pub title: String,
    pub items: Vec<DiagnosticItem>,
}

impl CategoryResult {
    pub fn new(category: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            title: title.into(),
            items: Vec::new(),
        }
    }

    pub fn overall_status(&self) -> Status {
        let mut highest = Status::Ok;
        for item in &self.items {
            match item.status {
                Status::Critical => return Status::Critical,
                Status::Error => highest = Status::Error,
                Status::Warning if highest != Status::Error => highest = Status::Warning,
                Status::Info if highest == Status::Ok => highest = Status::Info,
                _ => {}
            }
        }
        highest
    }
}

/// 診断サマリー統計
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DiagnosticSummary {
    pub ok: usize,
    pub info: usize,
    pub warning: usize,
    pub error: usize,
    pub critical: usize,
}

impl DiagnosticSummary {
    pub fn from_categories(categories: &[CategoryResult]) -> Self {
        let mut summary = DiagnosticSummary::default();
        for cat in categories {
            for item in &cat.items {
                match item.status {
                    Status::Ok => summary.ok += 1,
                    Status::Info => summary.info += 1,
                    Status::Warning => summary.warning += 1,
                    Status::Error => summary.error += 1,
                    Status::Critical => summary.critical += 1,
                }
            }
        }
        summary
    }

    pub fn total_issues(&self) -> usize {
        self.warning + self.error + self.critical
    }
}

/// サマリー表示用の個別 Issue 情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryIssue {
    pub category: String,
    pub item_name: String,
    pub status: Status,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_command: Option<String>,
}

/// 完全な診断レポート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullDiagnosticReport {
    pub version: String,
    pub timestamp: String,
    pub system: crate::system::SystemReport,
    pub summary: DiagnosticSummary,
    pub results: Vec<CategoryResult>,
}

impl FullDiagnosticReport {
    /// 診断レポート全体から全 Issue を抽出
    pub fn collect_issues(&self) -> Vec<SummaryIssue> {
        let mut issues = Vec::new();
        for cat in &self.results {
            for item in &cat.items {
                let first_cmd = item.recommendations.iter().find_map(|r| r.command.clone());
                for issue in &item.issues {
                    if !issue.status.is_ok() {
                        issues.push(SummaryIssue {
                            category: cat.title.clone(),
                            item_name: item.name.clone(),
                            status: issue.status,
                            message: issue.message.clone(),
                            cause: issue.cause.clone(),
                            impact: issue.impact.clone(),
                            fix_command: first_cmd.clone(),
                        });
                    }
                }
            }
        }
        issues
    }
}

/// チェッカーの種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckerKind {
    System,
    Language,
    Tool,
}

/// 各診断モジュールの共通トレイト
pub trait Checker: Send + Sync {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }
    /// 対象の言語やツールがシステム上にインストールされているか確認
    fn is_installed(&self) -> bool {
        true
    }
    /// 診断対象の別名 (例: "js", "ts", "javascript", "c#", ".net")
    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }
    fn check(&self) -> CategoryResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_severity_ordering() {
        let mut cat = CategoryResult::new("test", "Test Category");
        assert_eq!(cat.overall_status(), Status::Ok);

        cat.items.push(DiagnosticItem::ok("item1"));
        assert_eq!(cat.overall_status(), Status::Ok);

        cat.items.push(DiagnosticItem::warning("item2", "warn"));
        assert_eq!(cat.overall_status(), Status::Warning);

        cat.items.push(DiagnosticItem::error("item3", "err"));
        assert_eq!(cat.overall_status(), Status::Error);
    }

    #[test]
    fn test_summary_calculation() {
        let mut cat = CategoryResult::new("test", "Test Category");
        cat.items.push(DiagnosticItem::ok("ok1"));
        cat.items.push(DiagnosticItem::warning("warn1", "warn"));
        cat.items.push(DiagnosticItem::error("err1", "err"));

        let summary = DiagnosticSummary::from_categories(&[cat]);
        assert_eq!(summary.ok, 1);
        assert_eq!(summary.warning, 1);
        assert_eq!(summary.error, 1);
        assert_eq!(summary.total_issues(), 2);
    }
}

