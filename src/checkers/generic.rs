use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

/// 関連ツールの診断定義
#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub name: &'static str,
    pub executable: &'static str,
    pub version_args: &'static [&'static str],
}

/// 汎用プログラミング言語診断チェッカー
/// 任意の言語の実行ファイル、環境変数、周辺ツール、バージョン管理を宣言的に診断可能
#[derive(Debug, Clone)]
pub struct GenericLanguageChecker {
    pub id: &'static str,
    pub title: &'static str,
    pub primary_executables: Vec<&'static str>,
    pub version_args: Vec<&'static str>,
    pub aliases: Vec<&'static str>,
    pub env_vars: Vec<&'static str>,
    pub tools: Vec<ToolSpec>,
    pub version_managers: Vec<(&'static str, &'static str)>, // (name, env_var or binary)
    pub install_pkg_name: Option<&'static str>,
    pub install_guide_url: Option<&'static str>,
}

impl GenericLanguageChecker {
    pub fn new(id: &'static str, title: &'static str, primary_executable: &'static str) -> Self {
        Self {
            id,
            title,
            primary_executables: vec![primary_executable],
            version_args: vec!["--version"],
            aliases: Vec::new(),
            env_vars: Vec::new(),
            tools: Vec::new(),
            version_managers: Vec::new(),
            install_pkg_name: None,
            install_guide_url: None,
        }
    }

    pub fn with_executables(mut self, execs: &[&'static str]) -> Self {
        self.primary_executables = execs.to_vec();
        self
    }

    pub fn with_version_args(mut self, args: &[&'static str]) -> Self {
        self.version_args = args.to_vec();
        self
    }

    pub fn with_aliases(mut self, aliases: &[&'static str]) -> Self {
        self.aliases = aliases.to_vec();
        self
    }

    pub fn with_env_vars(mut self, envs: &[&'static str]) -> Self {
        self.env_vars = envs.to_vec();
        self
    }

    pub fn with_tool(mut self, name: &'static str, executable: &'static str, version_args: &'static [&'static str]) -> Self {
        self.tools.push(ToolSpec {
            name,
            executable,
            version_args,
        });
        self
    }

    pub fn with_version_manager(mut self, name: &'static str, identifier: &'static str) -> Self {
        self.version_managers.push((name, identifier));
        self
    }

    pub fn with_install_pkg(mut self, pkg: &'static str) -> Self {
        self.install_pkg_name = Some(pkg);
        self
    }

    pub fn with_install_guide(mut self, url: &'static str) -> Self {
        self.install_guide_url = Some(url);
        self
    }
}

impl Checker for GenericLanguageChecker {
    fn id(&self) -> &'static str {
        self.id
    }

    fn title(&self) -> &'static str {
        self.title
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        self.aliases.clone()
    }

    fn is_installed(&self) -> bool {
        self.primary_executables
            .iter()
            .any(|&exe| find_executable(exe).is_some())
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Primary Executable Check
        let mut found_exe = None;
        for &exe in &self.primary_executables {
            if let Some(path) = find_executable(exe) {
                found_exe = Some((exe, path));
                break;
            }
        }

        if let Some((_exe_name, path)) = found_exe {
            let path_str = path.to_string_lossy().to_string();
            let mut main_item = DiagnosticItem::ok(format!("{} Runtime / Compiler", self.title));
            main_item.path = Some(path_str.clone());

            let args_ref: Vec<&str> = self.version_args.iter().copied().collect();
            if let Some(ver) = run_cmd_first_line(&path_str, &args_ref) {
                main_item.version = Some(ver);
            }

            // Environment variables
            for &env_name in &self.env_vars {
                if let Ok(val) = std::env::var(env_name) {
                    main_item.details.push(format!("{}: {}", env_name, val));
                }
            }

            result.items.push(main_item);

            // 2. Related Tools (Package Managers, Build Tools, Linters)
            let mut detected_tools = Vec::new();
            for tool in &self.tools {
                if let Some(tool_path) = find_executable(tool.executable) {
                    let tool_str = tool_path.to_string_lossy().to_string();
                    let ver_out = run_cmd_first_line(&tool_str, tool.version_args);
                    if let Some(v) = ver_out {
                        detected_tools.push(format!("{} ({})", tool.name, v));
                    } else {
                        detected_tools.push(tool.name.to_string());
                    }
                }
            }

            if !detected_tools.is_empty() {
                let mut tools_item = DiagnosticItem::ok(format!("{} Tools & Package Managers", self.title));
                tools_item.details = detected_tools;
                result.items.push(tools_item);
            }

            // 3. Version Managers
            let mut detected_vms = Vec::new();
            for (vm_name, identifier) in &self.version_managers {
                if std::env::var(identifier).is_ok() || find_executable(identifier).is_some() {
                    detected_vms.push(vm_name.to_string());
                }
            }

            if !detected_vms.is_empty() {
                let mut vm_item = DiagnosticItem::ok(format!("{} Version Manager", self.title));
                vm_item.details = detected_vms;
                result.items.push(vm_item);
            }
        } else {
            let primary_name = self.primary_executables.first().copied().unwrap_or(self.id);
            let mut item = DiagnosticItem::ok(format!("{} Runtime / Compiler", self.title));
            item.status = crate::core::Status::Error;
            let mut issue = crate::core::Issue::new(
                crate::core::Status::Error,
                format!("'{}' executable was not found on PATH", primary_name),
            );
            issue.cause = Some(format!("{} compiler / runtime binary is not installed or not in PATH", self.title));
            issue.impact = Some(format!("{} source files and projects cannot be compiled or executed", self.title));
            item.issues.push(issue);

            if let Some(pkg) = self.install_pkg_name {
                if let Some(cmd) = crate::system::package_manager::get_install_command(pkg) {
                    item.recommendations.push(Recommendation::full(
                        format!("Install {} via package manager", self.title),
                        cmd,
                        format!("Installs the {} development environment.", self.title),
                    ));
                }
            }

            if let Some(url) = self.install_guide_url {
                item.recommendations.push(Recommendation::new(format!(
                    "For official installation instructions, see: {}",
                    url
                )));
            }

            result.items.push(item);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Status;

    #[test]
    fn test_generic_language_checker_builder() {
        let checker = GenericLanguageChecker::new("sample", "Sample Lang", "non_existent_binary_xyz_123")
            .with_aliases(&["smp"])
            .with_env_vars(&["SAMPLE_HOME"])
            .with_install_pkg("sample-lang");

        assert_eq!(checker.id(), "sample");
        assert_eq!(checker.kind(), CheckerKind::Language);
        assert!(!checker.is_installed());
        assert_eq!(checker.aliases(), vec!["smp"]);

        let res = checker.check();
        assert_eq!(res.category, "sample");
        assert_eq!(res.items.len(), 1);
        assert_eq!(res.items[0].status, Status::Error);
    }
}
