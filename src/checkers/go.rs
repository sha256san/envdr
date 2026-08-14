use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct GoChecker;

impl Checker for GoChecker {
    fn id(&self) -> &'static str {
        "go"
    }

    fn title(&self) -> &'static str {
        "Go Toolchain"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["golang"]
    }

    fn is_installed(&self) -> bool {
        find_executable("go").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Go Compiler / CLI
        if let Some(p) = find_executable("go") {
            let path_str = p.to_string_lossy().to_string();
            let mut go_item = DiagnosticItem::ok("Go CLI & Compiler");
            go_item.path = Some(path_str.clone());

            if let Some(v) = run_cmd_first_line(&path_str, &["version"]) {
                go_item.version = Some(v);
            }

            // GOPATH & GOROOT
            let gopath_env = std::env::var("GOPATH").ok();
            let goroot_env = std::env::var("GOROOT").ok();
            let go_env_gopath = run_cmd_first_line(&path_str, &["env", "GOPATH"]);
            let go_env_goroot = run_cmd_first_line(&path_str, &["env", "GOROOT"]);

            let active_gopath = gopath_env.or(go_env_gopath);
            let active_goroot = goroot_env.or(go_env_goroot);

            if let Some(ref gp) = active_gopath {
                go_item.details.push(format!("GOPATH: {}", gp));
            }
            if let Some(ref gr) = active_goroot {
                go_item.details.push(format!("GOROOT: {}", gr));
            }

            result.items.push(go_item);

            // 2. GOPATH/bin in PATH check
            if let Some(ref gp) = active_gopath {
                let bin_dir = std::path::Path::new(gp).join("bin");
                let mut bin_in_path = false;
                if let Some(path_os) = std::env::var_os("PATH") {
                    for p in std::env::split_paths(&path_os) {
                        if p == bin_dir {
                            bin_in_path = true;
                            break;
                        }
                    }
                }

                if !bin_in_path {
                    let mut path_warn = DiagnosticItem::ok("Go Bin Directory ($GOPATH/bin)");
                    path_warn.status = crate::core::Status::Warning;
                    path_warn.details.push(format!("GOPATH: {}", gp));
                    path_warn.details.push(format!("Binary directory: {}", bin_dir.display()));

                    let mut issue = crate::core::Issue::new(
                        crate::core::Status::Warning,
                        format!("{} is not in your PATH", bin_dir.display()),
                    );
                    issue.cause = Some("Go binary installation path is omitted from system PATH configuration".into());
                    issue.impact = Some("Binaries installed via 'go install' cannot be executed directly from your terminal".into());
                    path_warn.issues.push(issue);

                    let rec = Recommendation::with_command(
                        "Add GOPATH/bin to your PATH in your shell configuration (~/.bashrc or ~/.zshrc)",
                        "export PATH=\"$(go env GOPATH)/bin:$PATH\"",
                    )
                    .with_verification("echo $PATH | grep -q 'go/bin'");
                    path_warn.recommendations.push(rec);

                    result.items.push(path_warn);
                }
            }

            // 3. Go Tools & Linters (gofmt, golangci-lint, air)
            let mut tools = Vec::new();
            if let Some(p) = find_executable("gofmt") {
                tools.push(format!("gofmt ({})", p.display()));
            }
            if let Some(p) = find_executable("golangci-lint") {
                if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                    tools.push(format!("golangci-lint ({})", v));
                } else {
                    tools.push("golangci-lint".to_string());
                }
            }
            if let Some(p) = find_executable("air") {
                if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["-v"]) {
                    tools.push(format!("air (live reload) ({})", v));
                } else {
                    tools.push("air (live reload)".to_string());
                }
            }

            if !tools.is_empty() {
                let mut tool_item = DiagnosticItem::ok("Go Tools & Linters");
                tool_item.details = tools;
                result.items.push(tool_item);
            }
        } else {
            let mut go_item = DiagnosticItem::error(
                "Go Compiler",
                "go executable was not found on PATH",
            );
            let install_cmd = crate::system::package_manager::get_install_command("golang")
                .unwrap_or_else(|| "sudo apt install golang-go".to_string());
            go_item.recommendations.push(Recommendation::full(
                "Install Go programming language",
                install_cmd,
                "Go is required for Go project development.",
            ));
            result.items.push(go_item);
        }

        result
    }
}
