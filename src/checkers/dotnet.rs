use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::{run_cmd, run_cmd_first_line};
use crate::utils::path::find_executable;

pub struct DotnetChecker;

impl Checker for DotnetChecker {
    fn id(&self) -> &'static str {
        "dotnet"
    }

    fn title(&self) -> &'static str {
        "C# / .NET Toolchain"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["csharp", "c#", ".net"]
    }

    fn is_installed(&self) -> bool {
        find_executable("dotnet").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. dotnet CLI
        if let Some(p) = find_executable("dotnet") {
            let path_str = p.to_string_lossy().to_string();
            let mut dotnet_item = DiagnosticItem::ok(".NET CLI (dotnet)");
            dotnet_item.path = Some(path_str.clone());

            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                dotnet_item.version = Some(v);
            }

            if let Ok(root) = std::env::var("DOTNET_ROOT") {
                dotnet_item.details.push(format!("DOTNET_ROOT: {}", root));
            }

            result.items.push(dotnet_item);

            // 2. Installed SDKs
            let mut sdks_item = DiagnosticItem::ok(".NET SDKs");
            if let Some(out) = run_cmd(&path_str, &["--list-sdks"]) {
                if out.success && !out.stdout.is_empty() {
                    for line in out.stdout.lines() {
                        sdks_item.details.push(line.trim().to_string());
                    }
                } else {
                    sdks_item.details.push("No .NET SDKs installed (Runtime only)".to_string());
                }
            }
            result.items.push(sdks_item);

            // 3. Installed Runtimes
            let mut runtimes_item = DiagnosticItem::ok(".NET Runtimes");
            if let Some(out) = run_cmd(&path_str, &["--list-runtimes"]) {
                if out.success && !out.stdout.is_empty() {
                    for line in out.stdout.lines().take(5) {
                        runtimes_item.details.push(line.trim().to_string());
                    }
                    let count = out.stdout.lines().count();
                    if count > 5 {
                        runtimes_item.details.push(format!("... and {} more runtime(s)", count - 5));
                    }
                }
            }
            result.items.push(runtimes_item);
        } else {
            let mut dotnet_item = DiagnosticItem::error(
                ".NET Toolchain",
                "dotnet executable was not found on PATH",
            );
            let install_cmd = crate::system::package_manager::get_install_command("dotnet")
                .unwrap_or_else(|| "sudo apt install dotnet-sdk-8.0".to_string());
            dotnet_item.recommendations.push(Recommendation::full(
                "Install .NET SDK",
                install_cmd,
                ".NET SDK is required for C# and F# application development.",
            ));
            result.items.push(dotnet_item);
        }

        result
    }
}
