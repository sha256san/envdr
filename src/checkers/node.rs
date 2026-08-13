use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct NodeChecker;

impl Checker for NodeChecker {
    fn id(&self) -> &'static str {
        "node"
    }

    fn title(&self) -> &'static str {
        "Node.js & JavaScript Runtime"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["js", "ts", "javascript", "typescript", "npm", "yarn", "pnpm", "bun", "deno"]
    }

    fn is_installed(&self) -> bool {
        find_executable("node").is_some() || find_executable("bun").is_some() || find_executable("deno").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Node.js Runtime
        if let Some(p) = find_executable("node") {
            let path_str = p.to_string_lossy().to_string();
            let mut node_item = DiagnosticItem::ok("Node.js Runtime");
            node_item.path = Some(path_str.clone());
            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                node_item.version = Some(v);
            }
            result.items.push(node_item);

            // 2. Package Managers (npm, yarn, pnpm, bun)
            let mut pms = Vec::new();
            if let Some(npm_p) = find_executable("npm") {
                if let Some(v) = run_cmd_first_line(&npm_p.to_string_lossy(), &["--version"]) {
                    pms.push(format!("npm {}", v));
                }
            }
            if let Some(yarn_p) = find_executable("yarn") {
                if let Some(v) = run_cmd_first_line(&yarn_p.to_string_lossy(), &["--version"]) {
                    pms.push(format!("yarn {}", v));
                }
            }
            if let Some(pnpm_p) = find_executable("pnpm") {
                if let Some(v) = run_cmd_first_line(&pnpm_p.to_string_lossy(), &["--version"]) {
                    pms.push(format!("pnpm {}", v));
                }
            }
            if let Some(bun_p) = find_executable("bun") {
                if let Some(v) = run_cmd_first_line(&bun_p.to_string_lossy(), &["--version"]) {
                    pms.push(format!("bun {}", v));
                }
            }

            let mut pm_item = DiagnosticItem::ok("Package Managers");
            if !pms.is_empty() {
                pm_item.details = pms;
            } else {
                pm_item.status = Status::Warning;
                pm_item.issues.push(Issue::new(Status::Warning, "No package manager (npm/yarn/pnpm/bun) detected"));
            }
            result.items.push(pm_item);

            // 3. Node version managers check (nvm, fnm, volta)
            let mut managers = Vec::new();
            if std::env::var("NVM_DIR").is_ok() {
                managers.push("nvm (Node Version Manager)");
            }
            if std::env::var("FNM_DIR").is_ok() || find_executable("fnm").is_some() {
                managers.push("fnm (Fast Node Manager)");
            }
            if std::env::var("VOLTA_HOME").is_ok() || find_executable("volta").is_some() {
                managers.push("Volta");
            }
            if !managers.is_empty() {
                let mut mgr_item = DiagnosticItem::ok("Node Version Manager");
                mgr_item.details = managers.into_iter().map(|s| s.to_string()).collect();
                result.items.push(mgr_item);
            }
        } else {
            let mut node_item = DiagnosticItem::info(
                "Node.js Runtime",
            );
            node_item.details.push("Node.js is not installed (optional for non-JS/TS projects)".to_string());
            node_item.recommendations.push(Recommendation::full(
                "Install Node.js via fnm or nvm",
                "curl -fsSL https://fnm.vercel.app/install | bash && fnm install --lts",
                "fnm is a fast, cross-platform Node.js version manager.",
            ));
            result.items.push(node_item);
        }

        result
    }
}
