use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
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

        let mut rustc_ver_opt = None;
        let mut cargo_ver_opt = None;

        // 1. rustc compiler
        if let Some(p) = find_executable("rustc") {
            let path_str = p.to_string_lossy().to_string();
            let mut rustc_item = DiagnosticItem::ok("rustc (Compiler)");
            rustc_item.path = Some(path_str.clone());
            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                rustc_item.version = Some(v.clone());
                rustc_ver_opt = Some(v);
            }
            result.items.push(rustc_item);
        } else {
            let mut rustc_item = DiagnosticItem::ok("rustc (Compiler)");
            rustc_item.status = Status::Error;
            let mut issue = Issue::new(
                Status::Error,
                "rustc was not found on your PATH",
            );
            issue.cause = Some("Rust compiler is not installed or not exported to PATH".into());
            issue.impact = Some("Rust source code cannot be compiled".into());
            rustc_item.issues.push(issue);

            let rec = Recommendation::full(
                "Install Rust using rustup",
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
                "Official way to install Rust toolchain across platforms.",
            )
            .with_verification("rustc --version");
            rustc_item.recommendations.push(rec);
            result.items.push(rustc_item);
        }

        // 2. Cargo package manager
        if let Some(p) = find_executable("cargo") {
            let path_str = p.to_string_lossy().to_string();
            let mut cargo_item = DiagnosticItem::ok("Cargo (Build Tool)");
            cargo_item.path = Some(path_str.clone());
            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                cargo_item.version = Some(v.clone());
                cargo_ver_opt = Some(v);
            }
            result.items.push(cargo_item);
        } else {
            let mut cargo_item = DiagnosticItem::ok("Cargo (Build Tool)");
            cargo_item.status = Status::Error;
            let mut issue = Issue::new(
                Status::Error,
                "cargo was not found on your PATH",
            );
            issue.cause = Some("Cargo package manager is missing".into());
            issue.impact = Some("Rust crates cannot be built, tested, or managed".into());
            cargo_item.issues.push(issue);
            result.items.push(cargo_item);
        }

        // Check Version consistency between rustc and cargo
        if let (Some(r_ver), Some(c_ver)) = (rustc_ver_opt, cargo_ver_opt) {
            let r_num = r_ver.split_whitespace().nth(1);
            let c_num = c_ver.split_whitespace().nth(1);
            if let (Some(rn), Some(cn)) = (r_num, c_num) {
                if rn != cn {
                    let mut mismatch_item = DiagnosticItem::ok("Rust Toolchain Version Consistency");
                    mismatch_item.status = Status::Warning;
                    let mut issue = Issue::new(
                        Status::Warning,
                        format!("Version mismatch between rustc ({}) and cargo ({})", rn, cn),
                    );
                    issue.cause = Some("rustc and cargo originate from different installations or toolchains".into());
                    issue.impact = Some("May cause unexpected feature flags or crate compilation errors".into());
                    mismatch_item.issues.push(issue);

                    let rec = Recommendation::with_command(
                        "Update Rust toolchains to latest stable via rustup",
                        "rustup update",
                    )
                    .with_verification("rustc --version && cargo --version");
                    mismatch_item.recommendations.push(rec);
                    result.items.push(mismatch_item);
                }
            }
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
            let mut rustup_item = DiagnosticItem::ok("rustup (Toolchain Manager)");
            rustup_item.status = Status::Info;
            rustup_item.details.push("rustup is not installed (system-packaged Rust in use)".to_string());
            rustup_item.recommendations.push(Recommendation::new(
                "Consider installing rustup (https://rustup.rs) for multi-toolchain and cross-compilation support",
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
            let mut bin_item = DiagnosticItem::ok("Cargo Binary Path (~/.cargo/bin)");
            bin_item.status = Status::Warning;
            let mut issue = Issue::new(
                Status::Warning,
                "~/.cargo/bin is not in your PATH",
            );
            issue.cause = Some("Cargo executable directory is omitted from shell PATH variable".into());
            issue.impact = Some("Binaries installed via 'cargo install' cannot be executed directly by name".into());
            bin_item.issues.push(issue);

            let rec = Recommendation::with_command(
                "Add Cargo bin directory to your shell configuration (~/.bashrc or ~/.zshrc)",
                "export PATH=\"$HOME/.cargo/bin:$PATH\"",
            )
            .with_verification("echo $PATH | grep -q '.cargo/bin'");
            bin_item.recommendations.push(rec);
            result.items.push(bin_item);
        }

        result
    }
}
