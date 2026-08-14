use std::path::{Path, PathBuf};
use crate::utils::command::run_cmd;

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

/// パスがバージョン管理シム（pyenv, asdf, rbenv 等）のダミー空シムかどうかを判定
pub fn is_empty_shim(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    let is_shim_dir = path_str.contains("/.asdf/shims/")
        || path_str.contains("/.pyenv/shims/")
        || path_str.contains("/.rbenv/shims/")
        || path_str.contains("/.nodenv/shims/")
        || path_str.contains("/.jenv/shims/");

    if !is_shim_dir {
        return false;
    }

    // シムを実行して実体が存在するかテスト
    if let Some(out) = run_cmd(&path_str, &["--version"]) {
        let combined = format!("{} {}", out.stdout, out.stderr).to_lowercase();
        if combined.contains("not installed")
            || combined.contains("no version is set")
            || combined.contains("no version selected")
            || combined.contains("unknown command")
        {
            return true;
        }
    }

    false
}

/// 壊れていない有効な実行可能ファイルであるか検証
pub fn is_valid_executable(path: &Path) -> bool {
    if !path.exists() || !path.is_file() {
        return false;
    }

    // シンボリックリンクのリンク切れチェック
    if path.is_symlink() && std::fs::read_link(path).is_err() {
        return false;
    }

    // 空シムの除外
    if is_empty_shim(path) {
        return false;
    }

    true
}

/// 指定した実行可能ファイルが PATH 上に存在し、有効な実行可能ファイルであるか探索
pub fn find_executable(name: &str) -> Option<PathBuf> {
    if let Ok(path) = which::which(name) {
        if is_valid_executable(&path) {
            return Some(path);
        }
    }

    // which::which_all で2番目以降の有効な候補を探索
    if let Ok(all) = which::which_all(name) {
        for path in all {
            if is_valid_executable(&path) {
                return Some(path);
            }
        }
    }

    None
}

/// PATH 上のすべての有効な同名実行可能ファイルを探索（複数バージョン・競合検知用）
pub fn find_all_executables(name: &str) -> Vec<PathBuf> {
    let mut valid_paths = Vec::new();
    if let Ok(all) = which::which_all(name) {
        for path in all {
            if is_valid_executable(&path) && !valid_paths.contains(&path) {
                valid_paths.push(path);
            }
        }
    }
    valid_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_path_env() {
        let analyses = analyze_path_env();
        assert!(!analyses.is_empty(), "PATH analysis should find entries");
    }

    #[test]
    fn test_find_executable_standard_commands() {
        // 標準的な Linux/Unix コマンド
        let sh_path = find_executable("sh");
        assert!(sh_path.is_some(), "sh should be found on standard Unix");
    }

    #[test]
    fn test_is_valid_executable_nonexistent() {
        assert!(!is_valid_executable(Path::new("/nonexistent/binary/path")));
    }
}
