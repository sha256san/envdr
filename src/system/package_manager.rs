use crate::utils::path::find_executable;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Pacman,
    Apk,
    Zypper,
    Brew,
    Winget,
    Choco,
    Scoop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessLevel {
    Fresh,
    Stale,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct FreshnessReport {
    pub level: FreshnessLevel,
    pub days_since_update: Option<u64>,
    pub message: String,
    pub recommended_command: Option<String>,
}

impl PackageManager {
    pub fn detect() -> Option<Self> {
        if find_executable("brew").is_some() {
            return Some(PackageManager::Brew);
        }
        if find_executable("apt").is_some() || find_executable("apt-get").is_some() {
            return Some(PackageManager::Apt);
        }
        if find_executable("dnf").is_some() {
            return Some(PackageManager::Dnf);
        }
        if find_executable("pacman").is_some() {
            return Some(PackageManager::Pacman);
        }
        if find_executable("zypper").is_some() {
            return Some(PackageManager::Zypper);
        }
        if find_executable("apk").is_some() {
            return Some(PackageManager::Apk);
        }
        if find_executable("winget").is_some() {
            return Some(PackageManager::Winget);
        }
        if find_executable("choco").is_some() {
            return Some(PackageManager::Choco);
        }
        if find_executable("scoop").is_some() {
            return Some(PackageManager::Scoop);
        }
        None
    }

    pub fn name(&self) -> &'static str {
        match self {
            PackageManager::Apt => "apt",
            PackageManager::Dnf => "dnf",
            PackageManager::Pacman => "pacman",
            PackageManager::Apk => "apk",
            PackageManager::Zypper => "zypper",
            PackageManager::Brew => "brew",
            PackageManager::Winget => "winget",
            PackageManager::Choco => "choco",
            PackageManager::Scoop => "scoop",
        }
    }

    pub fn install_command(&self, pkg_name: &str) -> String {
        match self {
            PackageManager::Apt => format!("sudo apt update && sudo apt install -y {}", pkg_name),
            PackageManager::Dnf => format!("sudo dnf install -y {}", pkg_name),
            PackageManager::Pacman => format!("sudo pacman -S --noconfirm {}", pkg_name),
            PackageManager::Apk => format!("sudo apk add {}", pkg_name),
            PackageManager::Zypper => format!("sudo zypper install -y {}", pkg_name),
            PackageManager::Brew => format!("brew install {}", pkg_name),
            PackageManager::Winget => format!("winget install {}", pkg_name),
            PackageManager::Choco => format!("choco install -y {}", pkg_name),
            PackageManager::Scoop => format!("scoop install {}", pkg_name),
        }
    }

    /// パッケージマネージャーのキャッシュ鮮度・最終更新日時の診断
    pub fn check_freshness(&self) -> FreshnessReport {
        match self {
            PackageManager::Apt => check_apt_freshness(),
            PackageManager::Brew => check_brew_freshness(),
            PackageManager::Pacman => check_pacman_freshness(),
            PackageManager::Dnf => check_dnf_freshness(),
            _ => FreshnessReport {
                level: FreshnessLevel::Unknown,
                days_since_update: None,
                message: format!("Freshness check not implemented for {}", self.name()),
                recommended_command: None,
            },
        }
    }
}

/// APT のキャッシュ更新鮮度を判定（7日以上経過で警告）
fn check_apt_freshness() -> FreshnessReport {
    let candidate_paths = [
        "/var/lib/apt/periodic/update-success-stamp",
        "/var/cache/apt/pkgcache.bin",
        "/var/lib/apt/lists",
    ];

    let mut latest_mtime: Option<SystemTime> = None;
    for path in &candidate_paths {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(mtime) = metadata.modified() {
                latest_mtime = match latest_mtime {
                    Some(prev) if mtime > prev => Some(mtime),
                    None => Some(mtime),
                    _ => latest_mtime,
                };
            }
        }
    }

    if let Some(mtime) = latest_mtime {
        if let Ok(elapsed) = SystemTime::now().duration_since(mtime) {
            let days = elapsed.as_secs() / 86400;
            if days >= 7 {
                return FreshnessReport {
                    level: FreshnessLevel::Stale,
                    days_since_update: Some(days),
                    message: format!("APT package cache was updated {} day(s) ago (stale > 7 days)", days),
                    recommended_command: Some("sudo apt update".to_string()),
                };
            } else {
                return FreshnessReport {
                    level: FreshnessLevel::Fresh,
                    days_since_update: Some(days),
                    message: format!("APT package cache is up-to-date (updated {} day(s) ago)", days),
                    recommended_command: None,
                };
            }
        }
    }

    FreshnessReport {
        level: FreshnessLevel::Stale,
        days_since_update: None,
        message: "APT package cache timestamp not found or never updated".to_string(),
        recommended_command: Some("sudo apt update".to_string()),
    }
}

/// Homebrew の更新鮮度判定（14日以上経過で警告）
fn check_brew_freshness() -> FreshnessReport {
    let candidate_dirs = [
        "/opt/homebrew/.git/FETCH_HEAD",
        "/usr/local/Homebrew/.git/FETCH_HEAD",
        "/home/linuxbrew/.linuxbrew/Homebrew/.git/FETCH_HEAD",
    ];

    for path in &candidate_dirs {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(mtime) = metadata.modified() {
                if let Ok(elapsed) = SystemTime::now().duration_since(mtime) {
                    let days = elapsed.as_secs() / 86400;
                    if days >= 14 {
                        return FreshnessReport {
                            level: FreshnessLevel::Stale,
                            days_since_update: Some(days),
                            message: format!("Homebrew repository was updated {} day(s) ago (stale > 14 days)", days),
                            recommended_command: Some("brew update".to_string()),
                        };
                    } else {
                        return FreshnessReport {
                            level: FreshnessLevel::Fresh,
                            days_since_update: Some(days),
                            message: format!("Homebrew repository is up-to-date (updated {} day(s) ago)", days),
                            recommended_command: None,
                        };
                    }
                }
            }
        }
    }

    FreshnessReport {
        level: FreshnessLevel::Unknown,
        days_since_update: None,
        message: "Homebrew cache timestamp could not be verified directly".to_string(),
        recommended_command: Some("brew update".to_string()),
    }
}

/// Pacman の更新鮮度判定
fn check_pacman_freshness() -> FreshnessReport {
    let sync_dir = Path::new("/var/lib/pacman/sync");
    if let Ok(entries) = fs::read_dir(sync_dir) {
        let mut latest_mtime: Option<SystemTime> = None;
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(mtime) = metadata.modified() {
                    latest_mtime = match latest_mtime {
                        Some(prev) if mtime > prev => Some(mtime),
                        None => Some(mtime),
                        _ => latest_mtime,
                    };
                }
            }
        }

        if let Some(mtime) = latest_mtime {
            if let Ok(elapsed) = SystemTime::now().duration_since(mtime) {
                let days = elapsed.as_secs() / 86400;
                if days >= 14 {
                    return FreshnessReport {
                        level: FreshnessLevel::Stale,
                        days_since_update: Some(days),
                        message: format!("Pacman sync databases were updated {} day(s) ago (stale > 14 days)", days),
                        recommended_command: Some("sudo pacman -Sy".to_string()),
                    };
                } else {
                    return FreshnessReport {
                        level: FreshnessLevel::Fresh,
                        days_since_update: Some(days),
                        message: format!("Pacman sync databases are up-to-date (updated {} day(s) ago)", days),
                        recommended_command: None,
                    };
                }
            }
        }
    }

    FreshnessReport {
        level: FreshnessLevel::Unknown,
        days_since_update: None,
        message: "Pacman sync database not found".to_string(),
        recommended_command: Some("sudo pacman -Sy".to_string()),
    }
}

/// DNF の更新鮮度判定
fn check_dnf_freshness() -> FreshnessReport {
    let cache_dir = Path::new("/var/cache/dnf");
    if let Ok(metadata) = fs::metadata(cache_dir) {
        if let Ok(mtime) = metadata.modified() {
            if let Ok(elapsed) = SystemTime::now().duration_since(mtime) {
                let days = elapsed.as_secs() / 86400;
                if days >= 14 {
                    return FreshnessReport {
                        level: FreshnessLevel::Stale,
                        days_since_update: Some(days),
                        message: format!("DNF metadata cache was updated {} day(s) ago", days),
                        recommended_command: Some("sudo dnf makecache".to_string()),
                    };
                } else {
                    return FreshnessReport {
                        level: FreshnessLevel::Fresh,
                        days_since_update: Some(days),
                        message: format!("DNF metadata cache is up-to-date (updated {} day(s) ago)", days),
                        recommended_command: None,
                    };
                }
            }
        }
    }

    FreshnessReport {
        level: FreshnessLevel::Unknown,
        days_since_update: None,
        message: "DNF cache metadata not found".to_string(),
        recommended_command: Some("sudo dnf makecache".to_string()),
    }
}

/// 汎用パッケージの各ディストリビューション別名マッピング
pub fn get_install_command(generic_name: &str) -> Option<String> {
    let pm = PackageManager::detect()?;
    let pkg_name = match (pm, generic_name) {
        (PackageManager::Apt, "git") => "git",
        (PackageManager::Apt, "python3") => "python3 python3-pip python3-venv",
        (PackageManager::Apt, "build-essential") => "build-essential",
        (PackageManager::Apt, "docker") => "docker.io docker-compose-v2",
        (PackageManager::Apt, "cmake") => "cmake",
        (PackageManager::Apt, "golang") | (PackageManager::Apt, "go") => "golang-go",
        (PackageManager::Apt, "java") => "default-jdk",
        (PackageManager::Apt, "ruby") => "ruby-full",
        (PackageManager::Apt, "php") => "php-cli composer",
        (PackageManager::Apt, "dotnet") => "dotnet-sdk-8.0",
        (PackageManager::Apt, "lua") => "lua5.4 luarocks",

        (PackageManager::Dnf, "git") => "git",
        (PackageManager::Dnf, "python3") => "python3 python3-pip",
        (PackageManager::Dnf, "build-essential") => "gcc gcc-c++ make",
        (PackageManager::Dnf, "docker") => "docker-ce docker-compose-plugin",
        (PackageManager::Dnf, "cmake") => "cmake",
        (PackageManager::Dnf, "golang") | (PackageManager::Dnf, "go") => "golang",
        (PackageManager::Dnf, "java") => "java-latest-openjdk-devel",
        (PackageManager::Dnf, "ruby") => "ruby ruby-devel",
        (PackageManager::Dnf, "php") => "php-cli composer",
        (PackageManager::Dnf, "dotnet") => "dotnet-sdk-8.0",
        (PackageManager::Dnf, "lua") => "lua luarocks",

        (PackageManager::Pacman, "git") => "git",
        (PackageManager::Pacman, "python3") => "python python-pip",
        (PackageManager::Pacman, "build-essential") => "base-devel",
        (PackageManager::Pacman, "docker") => "docker docker-compose",
        (PackageManager::Pacman, "cmake") => "cmake",
        (PackageManager::Pacman, "golang") | (PackageManager::Pacman, "go") => "go",
        (PackageManager::Pacman, "java") => "jdk-openjdk",
        (PackageManager::Pacman, "ruby") => "ruby",
        (PackageManager::Pacman, "php") => "php composer",
        (PackageManager::Pacman, "dotnet") => "dotnet-sdk",
        (PackageManager::Pacman, "lua") => "lua luarocks",

        (PackageManager::Brew, "git") => "git",
        (PackageManager::Brew, "python3") => "python",
        (PackageManager::Brew, "build-essential") => "gcc make cmake",
        (PackageManager::Brew, "docker") => "docker docker-compose",
        (PackageManager::Brew, "cmake") => "cmake",
        (PackageManager::Brew, "golang") | (PackageManager::Brew, "go") => "go",
        (PackageManager::Brew, "java") => "openjdk",
        (PackageManager::Brew, "ruby") => "ruby",
        (PackageManager::Brew, "php") => "php composer",
        (PackageManager::Brew, "dotnet") => "dotnet",
        (PackageManager::Brew, "lua") => "lua luarocks",

        _ => generic_name,
    };

    Some(pm.install_command(pkg_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_manager_install_command() {
        let apt = PackageManager::Apt;
        assert_eq!(apt.install_command("git"), "sudo apt update && sudo apt install -y git");

        let brew = PackageManager::Brew;
        assert_eq!(brew.install_command("git"), "brew install git");
    }

    #[test]
    fn test_package_manager_freshness() {
        let apt = PackageManager::Apt;
        let report = apt.check_freshness();
        assert!(matches!(report.level, FreshnessLevel::Fresh | FreshnessLevel::Stale | FreshnessLevel::Unknown));
    }
}
