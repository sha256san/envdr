use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct PhpChecker;

impl Checker for PhpChecker {
    fn id(&self) -> &'static str {
        "php"
    }

    fn title(&self) -> &'static str {
        "PHP Environment"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["laravel", "symfony"]
    }

    fn is_installed(&self) -> bool {
        find_executable("php").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. PHP CLI
        if let Some(p) = find_executable("php") {
            let path_str = p.to_string_lossy().to_string();
            let mut php_item = DiagnosticItem::ok("PHP CLI");
            php_item.path = Some(path_str.clone());

            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                php_item.version = Some(v);
            }
            result.items.push(php_item);

            // 2. Composer Package Manager
            if let Some(comp_p) = find_executable("composer") {
                let mut comp_item = DiagnosticItem::ok("Composer (Package Manager)");
                comp_item.path = Some(comp_p.to_string_lossy().to_string());
                if let Some(v) = run_cmd_first_line(&comp_p.to_string_lossy(), &["--version"]) {
                    comp_item.version = Some(v);
                }
                result.items.push(comp_item);
            } else {
                let mut comp_item = DiagnosticItem::warning(
                    "Composer",
                    "Composer package manager was not found on PATH",
                );
                comp_item.recommendations.push(Recommendation::full(
                    "Install Composer",
                    "curl -sS https://getcomposer.org/installer | php && sudo mv composer.phar /usr/local/bin/composer",
                    "Composer is the dependency manager for PHP.",
                ));
                result.items.push(comp_item);
            }
        } else {
            let mut php_item = DiagnosticItem::error(
                "PHP CLI",
                "php executable was not found on PATH",
            );
            let install_cmd = crate::system::package_manager::get_install_command("php")
                .unwrap_or_else(|| "sudo apt install php-cli composer".to_string());
            php_item.recommendations.push(Recommendation::full(
                "Install PHP",
                install_cmd,
                "PHP runtime is required for PHP development.",
            ));
            result.items.push(php_item);
        }

        result
    }
}
