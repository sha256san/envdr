use crate::utils::path::find_executable;

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
}
