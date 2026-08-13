use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;
use std::path::PathBuf;

pub struct GitChecker;

impl Checker for GitChecker {
    fn id(&self) -> &'static str {
        "git"
    }

    fn title(&self) -> &'static str {
        "Git & Version Control"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Tool
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["vcs", "github"]
    }

    fn is_installed(&self) -> bool {
        find_executable("git").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Git command
        if let Some(p) = find_executable("git") {
            let path_str = p.to_string_lossy().to_string();
            let mut git_item = DiagnosticItem::ok("Git CLI");
            git_item.path = Some(path_str.clone());
            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                git_item.version = Some(v);
            }
            result.items.push(git_item);

            // 2. Git user.name & user.email
            let user_name = run_cmd_first_line(&path_str, &["config", "--get", "user.name"]);
            let user_email = run_cmd_first_line(&path_str, &["config", "--get", "user.email"]);

            let mut config_item = DiagnosticItem::ok("Git Author Configuration");
            match (&user_name, &user_email) {
                (Some(name), Some(email)) => {
                    config_item.details.push(format!("user.name: {}", name));
                    config_item.details.push(format!("user.email: {}", email));
                }
                _ => {
                    config_item.status = Status::Warning;
                    if user_name.is_none() {
                        config_item.issues.push(Issue::new(
                            Status::Warning,
                            "Git user.name is not set globally",
                        ));
                    }
                    if user_email.is_none() {
                        config_item.issues.push(Issue::new(
                            Status::Warning,
                            "Git user.email is not set globally",
                        ));
                    }
                    config_item.recommendations.push(Recommendation::full(
                        "Set Git global identity",
                        "git config --global user.name \"Your Name\" && git config --global user.email \"you@example.com\"",
                        "Required for commits to record proper authorship.",
                    ));
                }
            }
            result.items.push(config_item);

            // 3. SSH Keys for Git check (~/.ssh/id_* or ~/.ssh/config)
            let mut ssh_item = DiagnosticItem::ok("SSH Keys for Git / Remote");
            let mut found_keys = Vec::new();
            if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
                let ssh_dir = PathBuf::from(home).join(".ssh");
                if ssh_dir.exists() {
                    let key_names = ["id_ed25519", "id_rsa", "id_ecdsa"];
                    for k in key_names {
                        let priv_key = ssh_dir.join(k);
                        let pub_key = ssh_dir.join(format!("{}.pub", k));
                        if priv_key.exists() || pub_key.exists() {
                            found_keys.push(k.to_string());
                        }
                    }
                }
            }

            if !found_keys.is_empty() {
                ssh_item.details.push(format!("Found key(s): {}", found_keys.join(", ")));
            } else {
                ssh_item.status = Status::Info;
                ssh_item.issues.push(Issue::new(
                    Status::Info,
                    "No standard SSH keys (id_ed25519, id_rsa) detected in ~/.ssh/",
                ));
                ssh_item.recommendations.push(Recommendation::with_command(
                    "Generate an Ed25519 SSH key for GitHub/GitLab authentication",
                    "ssh-keygen -t ed25519 -C \"your_email@example.com\"",
                ));
            }
            // 4. Commit signing check (GPG / SSH)
            let gpg_sign = run_cmd_first_line(&path_str, &["config", "--get", "commit.gpgsign"]);
            let sign_key = run_cmd_first_line(&path_str, &["config", "--get", "user.signingkey"]);
            let mut signing_item = DiagnosticItem::ok("Git Commit Signing");
            if gpg_sign.as_deref() == Some("true") {
                signing_item.details.push("Commit signing is enabled (commit.gpgsign = true)".to_string());
                if let Some(key) = sign_key {
                    signing_item.details.push(format!("Signing key: {}", key));
                } else {
                    signing_item.status = Status::Warning;
                    signing_item.issues.push(Issue::new(
                        Status::Warning,
                        "commit.gpgsign is true, but user.signingkey is not configured",
                    ));
                }
            } else {
                signing_item.details.push("Commit signing is not enabled (optional for verified commits)".to_string());
            }
            result.items.push(signing_item);
        } else {
            let mut git_item = DiagnosticItem::error(
                "Git CLI",
                "git executable was not found on PATH",
            );
            let install_cmd = crate::system::package_manager::get_install_command("git")
                .unwrap_or_else(|| "sudo apt install git".to_string());
            git_item.recommendations.push(Recommendation::with_command(
                "Install Git",
                install_cmd,
            ));
            result.items.push(git_item);
        }

        result
    }
}
