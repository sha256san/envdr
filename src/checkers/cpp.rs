use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation, Status};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct CppChecker;

impl Checker for CppChecker {
    fn id(&self) -> &'static str {
        "cpp"
    }

    fn title(&self) -> &'static str {
        "C / C++ Toolchain"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["c", "c++", "gcc", "clang"]
    }

    fn is_installed(&self) -> bool {
        find_executable("gcc").is_some()
            || find_executable("clang").is_some()
            || find_executable("g++").is_some()
            || find_executable("clang++").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. C/C++ Compilers (gcc / clang)
        let mut compilers = Vec::new();
        if let Some(p) = find_executable("gcc") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                compilers.push(format!("GCC: {}", v));
            }
        }
        if let Some(p) = find_executable("g++") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                compilers.push(format!("G++: {}", v));
            }
        }
        if let Some(p) = find_executable("clang") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                compilers.push(format!("Clang: {}", v));
            }
        }
        if let Some(p) = find_executable("clang++") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                compilers.push(format!("Clang++: {}", v));
            }
        }

        let mut comp_item = DiagnosticItem::ok("C/C++ Compilers");
        if !compilers.is_empty() {
            comp_item.details = compilers;
        } else {
            comp_item.status = Status::Warning;
            comp_item.details.push("No C/C++ compiler found on PATH (gcc/clang)".to_string());
            let install_cmd = crate::system::package_manager::get_install_command("build-essential")
                .unwrap_or_else(|| "sudo apt install build-essential".to_string());
            comp_item.recommendations.push(Recommendation::with_command(
                "Install C/C++ compiler toolchain",
                install_cmd,
            ));
        }
        result.items.push(comp_item);

        // 2. Linkers (ld, lld, mold, gold)
        let mut linkers = Vec::new();
        if let Some(p) = find_executable("mold") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                linkers.push(format!("mold (High-speed linker): {}", v));
            }
        }
        if let Some(p) = find_executable("lld") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                linkers.push(format!("LLD (LLVM linker): {}", v));
            }
        }
        if let Some(p) = find_executable("ld") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                linkers.push(format!("GNU ld: {}", v));
            }
        }
        let mut linker_item = DiagnosticItem::ok("C/C++ & Rust Linkers");
        if !linkers.is_empty() {
            linker_item.details = linkers;
        } else {
            linker_item.status = Status::Warning;
            linker_item.details.push("No standard linker (ld/lld/mold) found".to_string());
        }
        result.items.push(linker_item);

        // 3. Build Systems (make, cmake, ninja, meson)
        let mut build_tools = Vec::new();
        if let Some(p) = find_executable("make") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                build_tools.push(format!("make: {}", v));
            }
        }
        if let Some(p) = find_executable("cmake") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                build_tools.push(format!("cmake: {}", v));
            }
        }
        if let Some(p) = find_executable("ninja") {
            if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                build_tools.push(format!("ninja: {}", v));
            }
        }

        let mut tools_item = DiagnosticItem::ok("C/C++ Build Systems");
        if !build_tools.is_empty() {
            tools_item.details = build_tools;
        } else {
            tools_item.status = Status::Info;
            tools_item.details.push("No build tools (make/cmake/ninja) detected".to_string());
        }
        result.items.push(tools_item);

        result
    }
}
