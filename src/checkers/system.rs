use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
use crate::system::package_manager::{FreshnessLevel, PackageManager};
use crate::system::SystemReport;
use crate::utils::path::analyze_path_env;

pub struct SystemChecker;

impl Checker for SystemChecker {
    fn id(&self) -> &'static str {
        "system"
    }

    fn title(&self) -> &'static str {
        "System & Environment"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::System
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["os", "path", "resources", "pm"]
    }

    fn is_installed(&self) -> bool {
        true
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());
        let sys = SystemReport::collect();

        // 1. OS & Architecture
        let mut os_item = DiagnosticItem::ok("OS & Architecture");
        let wsl_tag = if sys.is_wsl { " [WSL2]" } else { "" };
        os_item.version = Some(format!("{} {}{}", sys.os, sys.os_version, wsl_tag));
        os_item.details.push(format!("Architecture: {}", sys.arch));
        os_item.details.push(format!("Kernel: {}", sys.kernel_version));
        if sys.is_wsl {
            os_item.details.push("Environment: Windows Subsystem for Linux (WSL2)".to_string());
        }
        os_item.details.push(format!("Hostname: {}", sys.host_name));
        result.items.push(os_item);

        // 2. Hardware Resource
        let mut hw_item = DiagnosticItem::ok("Hardware Resources");
        hw_item.details.push(format!("CPU: {} ({} cores)", sys.cpu_brand, sys.cpu_count));
        hw_item.details.push(format!(
            "Memory: {} MB / {} MB used",
            sys.used_memory_mb, sys.total_memory_mb
        ));
        if sys.total_memory_mb < 2048 {
            hw_item.status = Status::Warning;
            let mut issue = Issue::new(
                Status::Warning,
                "Total system memory is less than 2GB, which may cause build failures",
            );
            issue.cause = Some("System is allocated limited RAM (< 2048MB)".into());
            issue.impact = Some("Heavy compilation (e.g., Rust/C++) may trigger Out-Of-Memory (OOM) killer".into());
            hw_item.issues.push(issue);
        }
        result.items.push(hw_item);

        // 3. PATH Environment Variable Analysis
        let path_analyses = analyze_path_env();
        let mut path_item = DiagnosticItem::ok("PATH Environment Variable");
        path_item.details.push(format!("Total entries in PATH: {}", path_analyses.len()));

        let mut linux_paths_count = 0;
        let mut windows_paths_count = 0;
        let mut non_existent_paths = Vec::new();
        let mut duplicate_paths = Vec::new();

        for entry in path_analyses {
            let path_str = entry.path.to_string_lossy().to_string();
            if path_str.starts_with("/mnt/c/") || path_str.starts_with("/mnt/d/") || path_str.contains(":\\") {
                windows_paths_count += 1;
            } else {
                linux_paths_count += 1;
            }

            if !entry.exists {
                non_existent_paths.push(path_str.clone());
            }
            if entry.is_duplicate {
                duplicate_paths.push(path_str);
            }
        }

        if sys.is_wsl && windows_paths_count > 0 {
            path_item.details.push(format!("Breakdown: {} Linux native paths, {} Windows mounted paths (/mnt/c/)", linux_paths_count, windows_paths_count));
        }

        if !non_existent_paths.is_empty() {
            path_item.status = Status::Warning;
            for p in &non_existent_paths {
                path_item.details.push(format!("Non-existent path: {}", p));
            }
            let mut issue = Issue::new(
                Status::Warning,
                format!("Found {} non-existent path(s) in PATH", non_existent_paths.len()),
            );
            issue.cause = if sys.is_wsl {
                Some("Directories specified in Linux or Windows imported PATH do not exist".into())
            } else {
                Some("Directories specified in PATH do not exist on the filesystem".into())
            };
            issue.impact = Some("Commands installed in these directories may fail to execute or produce confusing 'command not found' errors".into());
            path_item.issues.push(issue);

            let rec = Recommendation::new("Remove non-existent directories from your PATH in ~/.bashrc or ~/.zshrc")
                .with_verification("echo $PATH");
            path_item.recommendations.push(rec);
        }

        if !duplicate_paths.is_empty() {
            if path_item.status == Status::Ok {
                path_item.status = Status::Info;
            }
            for p in &duplicate_paths {
                path_item.details.push(format!("Duplicate path: {}", p));
            }
            let mut issue = Issue::new(
                Status::Info,
                format!("Found {} duplicate path(s) in PATH", duplicate_paths.len()),
            );
            issue.impact = Some("Redundant search paths slightly increase executable resolution time".into());
            path_item.issues.push(issue);
        }

        result.items.push(path_item);

        // 4. Package Manager & Cache Freshness
        if let Some(pm) = PackageManager::detect() {
            let freshness = pm.check_freshness();
            let mut pm_item = match freshness.level {
                FreshnessLevel::Fresh => DiagnosticItem::ok("Package Manager & Cache"),
                FreshnessLevel::Stale => {
                    let mut item = DiagnosticItem::ok("Package Manager & Cache");
                    item.status = Status::Warning;
                    let mut issue = Issue::new(Status::Warning, &freshness.message);
                    issue.cause = Some(format!("Package index for {} has not been updated recently", pm.name()));
                    issue.impact = Some("Installing packages may fetch outdated versions or fail with 404 Not Found errors".into());
                    item.issues.push(issue);

                    if let Some(cmd) = freshness.recommended_command {
                        let rec = Recommendation::with_command("Update package manager metadata cache", cmd.clone())
                            .with_verification(cmd);
                        item.recommendations.push(rec);
                    }
                    item
                }
                FreshnessLevel::Unknown => DiagnosticItem::ok("Package Manager & Cache"),
            };
            pm_item.details.push(format!("Active Package Manager: {}", pm.name()));
            pm_item.details.push(freshness.message);
            result.items.push(pm_item);
        } else {
            let mut pm_item = DiagnosticItem::info("Package Manager & Cache");
            pm_item.details.push("No recognized system package manager detected".to_string());
            result.items.push(pm_item);
        }

        result
    }
}
