use std::path::PathBuf;

/// PATH環境変数を分解し、各パスの検査結果を返す
pub struct PathEntryAnalysis {
    pub path: PathBuf,
    pub exists: bool,
    pub is_dir: bool,
    pub is_duplicate: bool,
}

pub fn analyze_path_env() -> Vec<PathEntryAnalysis> {
    let mut results = Vec::new();
    let mut seen = std::collections::HashSet::new();

    if let Some(path_os) = std::env::var_os("PATH") {
        for p in std::env::split_paths(&path_os) {
            let exists = p.exists();
            let is_dir = p.is_dir();
            let is_duplicate = !seen.insert(p.clone());

            results.push(PathEntryAnalysis {
                path: p,
                exists,
                is_dir,
                is_duplicate,
            });
        }
    }

    results
}

/// 指定した実行可能ファイルが PATH 上に存在するか探索
pub fn find_executable(name: &str) -> Option<PathBuf> {
    which::which(name).ok()
}
