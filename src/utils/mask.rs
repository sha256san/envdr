/// 機密情報の可能性がある環境変数名・設定キーを判定
pub fn is_sensitive_key(key: &str) -> bool {
    let upper = key.to_uppercase();
    let sensitive_keywords = [
        "KEY", "TOKEN", "SECRET", "PASSWORD", "PASSWD",
        "AUTH", "PRIVATE", "CREDENTIAL", "API", "ACCESS_KEY",
    ];

    sensitive_keywords.iter().any(|&keyword| upper.contains(keyword))
}

/// 機密値と思われる文字列をマスキング
pub fn mask_value(val: &str) -> String {
    if val.is_empty() {
        return String::new();
    }
    if val.len() <= 6 {
        return "******".to_string();
    }
    format!("{}******{}", &val[..2], &val[val.len() - 2..])
}

/// ホームディレクトリ等の実パスをチルダ `~` に置換
pub fn sanitize_path(path_str: &str) -> String {
    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        let home_str = home.to_string_lossy();
        if path_str.starts_with(home_str.as_ref()) {
            return path_str.replacen(home_str.as_ref(), "~", 1);
        }
    }
    path_str.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sensitive_key() {
        assert!(is_sensitive_key("OPENAI_API_KEY"));
        assert!(is_sensitive_key("GITHUB_TOKEN"));
        assert!(is_sensitive_key("DB_PASSWORD"));
        assert!(!is_sensitive_key("PATH"));
        assert!(!is_sensitive_key("HOME"));
    }

    #[test]
    fn test_mask_value() {
        assert_eq!(mask_value("short"), "******");
        assert_eq!(mask_value("secret123456"), "se******56");
    }
}

