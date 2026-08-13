use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct SwiftChecker;

impl Checker for SwiftChecker {
    fn id(&self) -> &'static str {
        "swift"
    }

    fn title(&self) -> &'static str {
        "Swift Toolchain"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["swiftc"]
    }

    fn is_installed(&self) -> bool {
        find_executable("swift").is_some() || find_executable("swiftc").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        if let Some(p) = find_executable("swift") {
            let path_str = p.to_string_lossy().to_string();
            let mut swift_item = DiagnosticItem::ok("Swift Compiler & Driver");
            swift_item.path = Some(path_str.clone());

            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                swift_item.version = Some(v);
            }
            result.items.push(swift_item);

            if let Some(swiftc_p) = find_executable("swiftc") {
                let mut swiftc_item = DiagnosticItem::ok("Swiftc (Standalone Compiler)");
                swiftc_item.path = Some(swiftc_p.to_string_lossy().to_string());
                result.items.push(swiftc_item);
            }
        } else {
            let mut swift_item = DiagnosticItem::error(
                "Swift Toolchain",
                "swift executable was not found on PATH",
            );
            swift_item.recommendations.push(Recommendation::new(
                "Install Swift toolchain from https://www.swift.org/install/",
            ));
            result.items.push(swift_item);
        }

        result
    }
}
