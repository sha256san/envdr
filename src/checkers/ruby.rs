use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct RubyChecker;

impl Checker for RubyChecker {
    fn id(&self) -> &'static str {
        "ruby"
    }

    fn title(&self) -> &'static str {
        "Ruby Environment"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["rb", "rails"]
    }

    fn is_installed(&self) -> bool {
        find_executable("ruby").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Ruby Interpreter
        if let Some(p) = find_executable("ruby") {
            let path_str = p.to_string_lossy().to_string();
            let mut ruby_item = DiagnosticItem::ok("Ruby Interpreter");
            ruby_item.path = Some(path_str.clone());

            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                ruby_item.version = Some(v);
            }
            result.items.push(ruby_item);

            // 2. RubyGems (gem)
            if let Some(gem_p) = find_executable("gem") {
                let mut gem_item = DiagnosticItem::ok("RubyGems Package Manager");
                gem_item.path = Some(gem_p.to_string_lossy().to_string());
                if let Some(v) = run_cmd_first_line(&gem_p.to_string_lossy(), &["--version"]) {
                    gem_item.version = Some(v);
                }
                result.items.push(gem_item);
            }

            // 3. Bundler (bundle)
            if let Some(bundle_p) = find_executable("bundle") {
                let mut bundle_item = DiagnosticItem::ok("Bundler");
                bundle_item.path = Some(bundle_p.to_string_lossy().to_string());
                if let Some(v) = run_cmd_first_line(&bundle_p.to_string_lossy(), &["--version"]) {
                    bundle_item.version = Some(v);
                }
                result.items.push(bundle_item);
            } else {
                let mut bundle_item = DiagnosticItem::warning(
                    "Bundler",
                    "Bundler ('bundle' command) is not installed",
                );
                bundle_item.recommendations.push(Recommendation::with_command(
                    "Install Bundler gem",
                    "gem install bundler",
                ));
                result.items.push(bundle_item);
            }

            // 4. Version Managers (rbenv, rvm, asdf, chruby)
            let mut vms = Vec::new();
            if std::env::var("RBENV_ROOT").is_ok() || find_executable("rbenv").is_some() {
                vms.push("rbenv");
            }
            if std::env::var("rvm_path").is_ok() || find_executable("rvm").is_some() {
                vms.push("RVM (Ruby Version Manager)");
            }
            if std::env::var("ASDF_DATA_DIR").is_ok() || find_executable("asdf").is_some() {
                vms.push("asdf");
            }
            if !vms.is_empty() {
                let mut vm_item = DiagnosticItem::ok("Ruby Version Manager");
                vm_item.details = vms.into_iter().map(|s| s.to_string()).collect();
                result.items.push(vm_item);
            }
        } else {
            let mut ruby_item = DiagnosticItem::error(
                "Ruby Interpreter",
                "ruby executable was not found on PATH",
            );
            let install_cmd = crate::system::package_manager::get_install_command("ruby")
                .unwrap_or_else(|| "sudo apt install ruby-full".to_string());
            ruby_item.recommendations.push(Recommendation::full(
                "Install Ruby",
                install_cmd,
                "Ruby interpreter is required for Ruby and Rails development.",
            ));
            result.items.push(ruby_item);
        }

        result
    }
}
