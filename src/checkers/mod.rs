pub mod cpp;
pub mod dart;
pub mod docker;
pub mod dotnet;
pub mod generic;
pub mod git;
pub mod go;
pub mod gpu;
pub mod java;
pub mod lua;
pub mod node;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod shell;
pub mod swift;
pub mod system;
pub mod zig;

use crate::core::{CategoryResult, Checker, CheckerKind};
use std::sync::Arc;

pub struct CheckerRegistry {
    checkers: Vec<Arc<dyn Checker>>,
}

impl Default for CheckerRegistry {
    fn default() -> Self {
        let mut registry = Self {
            checkers: Vec::new(),
        };

        // System checkers
        registry.register(Arc::new(system::SystemChecker));
        registry.register(Arc::new(shell::ShellChecker));

        // Language checkers
        registry.register(Arc::new(python::PythonChecker));
        registry.register(Arc::new(rust::RustChecker));
        registry.register(Arc::new(node::NodeChecker));
        registry.register(Arc::new(go::GoChecker));
        registry.register(Arc::new(cpp::CppChecker));
        registry.register(Arc::new(java::JavaChecker));
        registry.register(Arc::new(ruby::RubyChecker));
        registry.register(Arc::new(php::PhpChecker));
        registry.register(Arc::new(dotnet::DotnetChecker));
        registry.register(Arc::new(swift::SwiftChecker));
        registry.register(Arc::new(dart::DartChecker));
        registry.register(Arc::new(zig::ZigChecker));
        registry.register(Arc::new(lua::LuaChecker));

        // Tool checkers
        registry.register(Arc::new(git::GitChecker));
        registry.register(Arc::new(docker::DockerChecker));
        registry.register(Arc::new(gpu::GpuChecker));

        registry
    }
}

impl CheckerRegistry {
    pub fn register(&mut self, checker: Arc<dyn Checker>) {
        self.checkers.push(checker);
    }

    /// 全体診断: インストールされていない言語は自動的に除外（非表示）して実行
    pub fn run_all(&self) -> Vec<CategoryResult> {
        self.checkers
            .iter()
            .filter(|c| {
                // システムおよびツールは常に診断。言語はインストールされているもののみ診断
                match c.kind() {
                    CheckerKind::Language => c.is_installed(),
                    _ => true,
                }
            })
            .map(|c| c.check())
            .collect()
    }

    /// 言語診断: targets が指定されていれば未インストールでも診断、未指定時はインストール済み言語のみ診断
    pub fn run_languages(&self, targets: Option<&[String]>) -> Vec<CategoryResult> {
        if let Some(target_list) = targets {
            if target_list.is_empty() || target_list.iter().any(|t| t.eq_ignore_ascii_case("all")) {
                // 全言語（インストール済みのみ）
                return self
                    .checkers
                    .iter()
                    .filter(|c| c.kind() == CheckerKind::Language && c.is_installed())
                    .map(|c| c.check())
                    .collect();
            }

            // 指定された言語を診断
            self.run_multiple_targets(target_list)
        } else {
            // 言語のみすべて（インストール済みのみ）
            self.checkers
                .iter()
                .filter(|c| c.kind() == CheckerKind::Language && c.is_installed())
                .map(|c| c.check())
                .collect()
        }
    }

    /// ツール診断: targets が指定されていれば該当ツール、未指定時は全ツール
    pub fn run_tools(&self, targets: Option<&[String]>) -> Vec<CategoryResult> {
        if let Some(target_list) = targets {
            if target_list.is_empty() || target_list.iter().any(|t| t.eq_ignore_ascii_case("all")) {
                return self
                    .checkers
                    .iter()
                    .filter(|c| matches!(c.kind(), CheckerKind::Tool | CheckerKind::System))
                    .map(|c| c.check())
                    .collect();
            }

            self.run_multiple_targets(target_list)
        } else {
            self.checkers
                .iter()
                .filter(|c| matches!(c.kind(), CheckerKind::Tool | CheckerKind::System))
                .map(|c| c.check())
                .collect()
        }
    }

    /// 単一ターゲットの検索＆診断（IDまたは別名でマッチ）
    pub fn run_target(&self, target: &str) -> Option<CategoryResult> {
        self.find_checker(target).map(|c| c.check())
    }

    /// 複数ターゲットの検索＆診断（重複排除）
    pub fn run_multiple_targets(&self, targets: &[String]) -> Vec<CategoryResult> {
        let mut executed_ids = std::collections::HashSet::new();
        let mut results = Vec::new();

        for target in targets {
            // カンマ区切りの可能性にも対応
            for sub_target in target.split(',') {
                let trimmed = sub_target.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if let Some(checker) = self.find_checker(trimmed) {
                    if executed_ids.insert(checker.id()) {
                        results.push(checker.check());
                    }
                } else {
                    eprintln!("Warning: Unknown diagnostic target '{}'.", trimmed);
                }
            }
        }

        results
    }

    pub fn find_checker(&self, query: &str) -> Option<Arc<dyn Checker>> {
        let query_lower = query.to_lowercase();
        self.checkers.iter().find(|c| {
            c.id() == query_lower
                || c.aliases().iter().any(|a| a.to_lowercase() == query_lower)
        }).cloned()
    }

    pub fn available_targets(&self) -> Vec<&'static str> {
        self.checkers.iter().map(|c| c.id()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_targets_lookup() {
        let registry = CheckerRegistry::default();
        assert!(registry.find_checker("python").is_some());
        assert!(registry.find_checker("py").is_some());
        assert!(registry.find_checker("rust").is_some());
        assert!(registry.find_checker("cargo").is_some());
        assert!(registry.find_checker("go").is_some());
        assert!(registry.find_checker("golang").is_some());
        assert!(registry.find_checker("java").is_some());
        assert!(registry.find_checker("docker").is_some());
        assert!(registry.find_checker("gpu").is_some());
        assert!(registry.find_checker("cuda").is_some());
        assert!(registry.find_checker("ruby").is_some());
        assert!(registry.find_checker("php").is_some());
        assert!(registry.find_checker("dotnet").is_some());
        assert!(registry.find_checker("csharp").is_some());
        assert!(registry.find_checker("swift").is_some());
        assert!(registry.find_checker("dart").is_some());
        assert!(registry.find_checker("zig").is_some());
        assert!(registry.find_checker("lua").is_some());
    }

    #[test]
    fn test_registry_run_all_excludes_uninstalled_languages() {
        let registry = CheckerRegistry::default();
        let results = registry.run_all();
        // システムとシェルは常に入っている
        assert!(results.iter().any(|r| r.category == "system"));
        assert!(results.iter().any(|r| r.category == "shell"));
    }

    #[test]
    fn test_registry_multiple_targets() {
        let registry = CheckerRegistry::default();
        let targets = vec!["python".to_string(), "rust".to_string(), "py".to_string()];
        let results = registry.run_multiple_targets(&targets);
        // "python" and "py" point to same checker, so python is not duplicated
        let python_count = results.iter().filter(|r| r.category == "python").count();
        assert_eq!(python_count, 1);
        let rust_count = results.iter().filter(|r| r.category == "rust").count();
        assert_eq!(rust_count, 1);
    }
}
