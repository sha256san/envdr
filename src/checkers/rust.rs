use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct RustChecker;

impl Checker for RustChecker {
    fn id(&self) -> &'static str {
        "rust"
    }

    fn title(&self) -> &'static str {
        "Rust Toolchain"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["cargo", "rustc", "rustup"]
    }

    fn is_installed(&self) -> bool {
        find_executable("rustc").is_some() || find_executable("cargo").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. rustc compiler
        if let Some(p) = find_executable("rustc") {
            let path_str = p.to_string_lossy().to_string();
            let mut rustc_item = DiagnosticItem::ok("rustc (Compiler)");
            rustc_item.path = Some(path_str.clone());
            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                rustc_item.version = Some(v);
            }
            result.items.push(rustc_item);
        } else {
            let mut rustc_item = DiagnosticItem::error(
                "rustc (Compiler)",
                "rustc was not found on your PATH",
            );
            rustc_item.recommendations.push(Recommendation::full(
                "Install Rust using rustup",
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
                "Official way to install Rust toolchain across platforms.",
            ));
            result.items.push(rustc_item);
        }

        // 2. Cargo package manager
        if let Some(p) = find_executable("cargo") {
            let path_str = p.to_string_lossy().to_string();
            let mut cargo_item = DiagnosticItem::ok("Cargo (Build Tool)");
            cargo_item.path = Some(path_str.clone());
            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                cargo_item.version = Some(v);
            }
            result.items.push(cargo_item);
        } else {
            let cargo_item = DiagnosticItem::error(
                "Cargo (Build Tool)",
                "cargo was not found on your PATH",
            );
            result.items.push(cargo_item);
        }

        // 3. Rustup toolchain manager
        if let Some(p) = find_executable("rustup") {
            let path_str = p.to_string_lossy().to_string();
            let mut rustup_item = DiagnosticItem::ok("rustup (Toolchain Manager)");
            rustup_item.path = Some(path_str.clone());
            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                rustup_item.version = Some(v);
            }
            if let Some(default_tc) = run_cmd_first_line(&path_str, &["default"]) {
                rustup_item.details.push(format!("Active Toolchain: {}", default_tc));
            }
            result.items.push(rustup_item);
        } else {
            let mut rustup_item = DiagnosticItem::warning(
                "rustup (Toolchain Manager)",
                "rustup is not installed or not in PATH (system-packaged Rust may be in use)",
            );
            rustup_item.recommendations.push(Recommendation::new(
                "Consider using rustup to manage Rust toolchains and targets seamlessly",
            ));
            result.items.push(rustup_item);
        }

        // 4. ~/.cargo/bin in PATH check
        let cargo_bin_in_path = if let Some(path_os) = std::env::var_os("PATH") {
            let mut found = false;
            for p in std::env::split_paths(&path_os) {
                let p_str = p.to_string_lossy();
                if p_str.ends_with(".cargo/bin") || p_str.ends_with(".cargo\\bin") {
                    found = true;
                    break;
                }
            }
            found
        } else {
            false
        };

        if !cargo_bin_in_path {
            let mut bin_item = DiagnosticItem::warning(
                "Cargo Binary Path (~/.cargo/bin)",
                "~/.cargo/bin is not in your PATH. Binaries installed via 'cargo install' won't be directly executable.",
            );
            bin_item.recommendations.push(Recommendation::with_command(
                "Add Cargo bin directory to your shell configuration",
                "export PATH=\"$HOME/.cargo/bin:$PATH\"",
            ));
            result.items.push(bin_item);
        }

        result
    }
}
