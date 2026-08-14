use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
use crate::utils::command::{run_cmd, run_cmd_first_line};
use crate::utils::path::find_executable;
use std::path::Path;

pub struct GpuChecker;

impl Checker for GpuChecker {
    fn id(&self) -> &'static str {
        "gpu"
    }

    fn title(&self) -> &'static str {
        "GPU & Acceleration (CUDA / ROCm)"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Tool
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["cuda", "rocm", "nvidia", "amd"]
    }

    fn is_installed(&self) -> bool {
        find_executable("nvidia-smi").is_some()
            || find_executable("rocm-smi").is_some()
            || find_executable("rocminfo").is_some()
            || Path::new("/dev/kfd").exists()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());
        let mut gpu_found = false;

        // 1. NVIDIA GPU / CUDA Check
        if let Some(p) = find_executable("nvidia-smi") {
            gpu_found = true;
            let path_str = p.to_string_lossy().to_string();
            let mut nvidia_item = DiagnosticItem::ok("NVIDIA GPU & Driver");
            nvidia_item.path = Some(path_str.clone());

            let smi_out = run_cmd(&path_str, &["--query-gpu=name,driver_version,memory.total", "--format=csv,noheader"]);
            if let Some(out) = smi_out {
                if out.success && !out.stdout.is_empty() {
                    for line in out.stdout.lines() {
                        nvidia_item.details.push(format!("GPU: {}", line.trim()));
                    }
                } else {
                    nvidia_item.status = Status::Warning;
                    nvidia_item.issues.push(Issue::new(
                        Status::Warning,
                        "nvidia-smi failed to communicate with NVIDIA driver",
                    ));
                    nvidia_item.recommendations.push(Recommendation::new(
                        "Check if NVIDIA driver is loaded: sudo modprobe nvidia or reboot",
                    ));
                }
            }

            // Check nvcc (CUDA Toolkit)
            if let Some(nvcc_p) = find_executable("nvcc") {
                if let Some(nvcc_ver) = run_cmd_first_line(&nvcc_p.to_string_lossy(), &["--version"]) {
                    nvidia_item.details.push(format!("CUDA Toolkit (nvcc): {}", nvcc_ver));
                }
            } else {
                nvidia_item.details.push("CUDA Toolkit (nvcc) not found in PATH (optional for runtime inference)".to_string());
            }

            result.items.push(nvidia_item);
        }

        // 2. AMD ROCm Check
        let rocm_smi = find_executable("rocm-smi");
        let rocminfo = find_executable("rocminfo");
        let kfd_exists = Path::new("/dev/kfd").exists();

        if rocm_smi.is_some() || rocminfo.is_some() || kfd_exists {
            gpu_found = true;
            let mut rocm_item = DiagnosticItem::ok("AMD ROCm GPU Acceleration");

            if let Some(smi_p) = rocm_smi {
                rocm_item.path = Some(smi_p.to_string_lossy().to_string());
                if let Some(v) = run_cmd_first_line(&smi_p.to_string_lossy(), &["--showdriverversion"]) {
                    rocm_item.details.push(v);
                }
            }

            // Check /dev/kfd permission
            if kfd_exists {
                rocm_item.details.push("Found /dev/kfd (Kernel Fusion Driver device)".to_string());
            } else {
                rocm_item.status = Status::Warning;
                rocm_item.issues.push(Issue::new(
                    Status::Warning,
                    "/dev/kfd does not exist. ROCm kernel driver may not be loaded.",
                ));
            }

            // Check user group membership (render / video)
            let user_groups = run_cmd_first_line("groups", &[]);
            if let Some(groups) = user_groups {
                let has_render = groups.contains("render");
                let has_video = groups.contains("video");
                if !has_render && !has_video {
                    rocm_item.status = Status::Warning;
                    let mut issue = Issue::new(
                        Status::Warning,
                        "Current user is not in 'render' or 'video' groups for ROCm GPU access",
                    );
                    issue.cause = Some("Active user lacks UNIX group privileges for /dev/kfd and GPU device nodes".into());
                    issue.impact = Some("ROCm compute applications and PyTorch/TensorFlow cannot execute on AMD GPU hardware".into());
                    rocm_item.issues.push(issue);

                    let rec = Recommendation::full(
                        "Add user to render and video groups",
                        "sudo usermod -aG render,video $USER && newgrp render",
                        "Enables direct GPU memory and hardware queue access.",
                    )
                    .with_verification("rocm-smi");
                    rocm_item.recommendations.push(rec);
                } else {
                    rocm_item.details.push("User has proper GPU group permissions (render/video)".to_string());
                }
            }

            result.items.push(rocm_item);
        }

        if !gpu_found {
            let mut no_gpu = DiagnosticItem::info("GPU / Hardware Acceleration");
            no_gpu.details.push("No dedicated NVIDIA or AMD ROCm GPU detected (CPU mode)".to_string());
            result.items.push(no_gpu);
        }

        result
    }
}
