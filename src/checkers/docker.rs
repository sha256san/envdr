use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
use crate::utils::command::{run_cmd, run_cmd_first_line};
use crate::utils::path::find_executable;

pub struct DockerChecker;

impl Checker for DockerChecker {
    fn id(&self) -> &'static str {
        "docker"
    }

    fn title(&self) -> &'static str {
        "Docker & Containerization"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Tool
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["containers", "container"]
    }

    fn is_installed(&self) -> bool {
        find_executable("docker").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Docker CLI
        if let Some(p) = find_executable("docker") {
            let path_str = p.to_string_lossy().to_string();
            let mut docker_item = DiagnosticItem::ok("Docker CLI");
            docker_item.path = Some(path_str.clone());
            if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                docker_item.version = Some(v);
            }
            result.items.push(docker_item);

            // 2. Docker Daemon & Socket Permissions
            let daemon_test = run_cmd(&path_str, &["info"]);
            let mut daemon_item = DiagnosticItem::ok("Docker Daemon & Permissions");

            match daemon_test {
                Some(out) if out.success => {
                    daemon_item.details.push("Docker daemon is running and accessible".to_string());
                }
                Some(out) => {
                    daemon_item.status = Status::Error;
                    if out.stderr.contains("permission denied") || out.stderr.contains("Got permission denied") {
                        daemon_item.issues.push(Issue::new(
                            Status::Error,
                            "Current user does not have permission to communicate with Docker daemon socket (/var/run/docker.sock)",
                        ));
                        daemon_item.recommendations.push(Recommendation::full(
                            "Add current user to 'docker' group",
                            "sudo usermod -aG docker $USER && newgrp docker",
                            "Enables non-root access to the Docker daemon.",
                        ));
                    } else if out.stderr.contains("Is the docker daemon running") {
                        daemon_item.issues.push(Issue::new(
                            Status::Error,
                            "Docker daemon is not running",
                        ));
                        daemon_item.recommendations.push(Recommendation::with_command(
                            "Start Docker daemon service",
                            "sudo systemctl start docker",
                        ));
                    } else {
                        daemon_item.issues.push(Issue::new(
                            Status::Error,
                            format!("Docker daemon check failed: {}", out.stderr),
                        ));
                    }
                }
                None => {
                    daemon_item.status = Status::Error;
                    daemon_item.issues.push(Issue::new(Status::Error, "Failed to execute 'docker info'"));
                }
            }
            result.items.push(daemon_item);

            // 3. Docker Compose (v2 plugin or v1 standalone)
            let mut compose_item = DiagnosticItem::ok("Docker Compose");
            let compose_v2 = run_cmd_first_line(&path_str, &["compose", "version"]);
            if let Some(v2) = compose_v2 {
                compose_item.version = Some(v2);
                compose_item.details.push("Using Docker Compose V2 (CLI plugin)".to_string());
            } else if let Some(v1_path) = find_executable("docker-compose") {
                if let Some(v1) = run_cmd_first_line(&v1_path.to_string_lossy(), &["--version"]) {
                    compose_item.version = Some(v1);
                    compose_item.status = Status::Info;
                    compose_item.details.push("Using Docker Compose V1 (legacy standalone)".to_string());
                    compose_item.recommendations.push(Recommendation::new(
                        "Consider upgrading to Docker Compose V2 ('docker compose')",
                    ));
                }
            } else {
                compose_item.status = Status::Info;
                compose_item.details.push("Docker Compose not found".to_string());
            }
            result.items.push(compose_item);

            // 4. GPU Container Passthrough (NVIDIA Container Toolkit)
            if find_executable("nvidia-ctk").is_some() || find_executable("nvidia-container-runtime").is_some() {
                let mut gpu_ctk = DiagnosticItem::ok("Docker GPU Passthrough");
                gpu_ctk.details.push("NVIDIA Container Toolkit is installed".to_string());
                result.items.push(gpu_ctk);
            }
        } else {
            let mut docker_item = DiagnosticItem::info("Docker CLI");
            docker_item.details.push("Docker is not installed on this system".to_string());
            docker_item.recommendations.push(Recommendation::with_command(
                "Install Docker Engine",
                "https://docs.docker.com/engine/install/",
            ));
            result.items.push(docker_item);
        }

        result
    }
}
