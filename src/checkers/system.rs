use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
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
        vec!["os", "path", "resources"]
    }

    fn is_installed(&self) -> bool {
        true
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());
        let sys = SystemReport::collect();

        // 1. OS & Architecture
        let mut os_item = DiagnosticItem::ok("OS & Architecture");
        os_item.version = Some(format!("{} {} ({})", sys.os, sys.os_version, sys.arch));
        os_item.details.push(format!("Kernel: {}", sys.kernel_version));
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
            hw_item.issues.push(Issue::new(
                Status::Warning,
                "Total system memory is less than 2GB, which may cause build failures",
            ));
        }
        result.items.push(hw_item);

        // 3. PATH Environment Variable Analysis
        let path_analyses = analyze_path_env();
        let mut path_item = DiagnosticItem::ok("PATH Environment Variable");
        path_item.details.push(format!("Total entries in PATH: {}", path_analyses.len()));

        let mut non_existent_paths = Vec::new();
        let mut duplicate_paths = Vec::new();

        for entry in path_analyses {
            let path_str = entry.path.to_string_lossy().to_string();
            if !entry.exists {
                non_existent_paths.push(path_str.clone());
            }
            if entry.is_duplicate {
                duplicate_paths.push(path_str);
            }
        }

        if !non_existent_paths.is_empty() {
            path_item.status = Status::Warning;
            path_item.issues.push(Issue::with_detail(
                Status::Warning,
                format!("Found {} non-existent path(s) in PATH", non_existent_paths.len()),
                non_existent_paths.join(", "),
            ));
            path_item.recommendations.push(Recommendation::new(
                "Remove non-existent directories from your PATH in ~/.bashrc or ~/.zshrc",
            ));
        }

        if !duplicate_paths.is_empty() {
            if path_item.status == Status::Ok {
                path_item.status = Status::Info;
            }
            path_item.issues.push(Issue::with_detail(
                Status::Info,
                format!("Found {} duplicate path(s) in PATH", duplicate_paths.len()),
                duplicate_paths.join(", "),
            ));
        }

        result.items.push(path_item);

        result
    }
}
