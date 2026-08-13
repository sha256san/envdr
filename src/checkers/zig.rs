use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct ZigChecker;

impl Checker for ZigChecker {
    fn id(&self) -> &'static str {
        "zig"
    }

    fn title(&self) -> &'static str {
        "Zig Toolchain"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn is_installed(&self) -> bool {
        find_executable("zig").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        if let Some(p) = find_executable("zig") {
            let path_str = p.to_string_lossy().to_string();
            let mut zig_item = DiagnosticItem::ok("Zig Compiler");
            zig_item.path = Some(path_str.clone());

            if let Some(v) = run_cmd_first_line(&path_str, &["version"]) {
                zig_item.version = Some(v);
            }
            result.items.push(zig_item);
        } else {
            let mut zig_item = DiagnosticItem::error(
                "Zig Compiler",
                "zig executable was not found on PATH",
            );
            zig_item.recommendations.push(Recommendation::new(
                "Install Zig compiler from https://ziglang.org/download/",
            ));
            result.items.push(zig_item);
        }

        result
    }
}
