use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct PythonChecker;

impl Checker for PythonChecker {
    fn id(&self) -> &'static str {
        "python"
    }

    fn title(&self) -> &'static str {
        "Python Environment"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["py", "python3"]
    }

    fn is_installed(&self) -> bool {
        find_executable("python3").is_some() || find_executable("python").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Python 3 Interpreter Check
        let python3_path = find_executable("python3").or_else(|| find_executable("python"));
        if let Some(p) = python3_path {
            let path_str = p.to_string_lossy().to_string();
            let mut py_item = DiagnosticItem::ok("Python Interpreter");
            py_item.path = Some(path_str.clone());

            if let Some(ver) = run_cmd_first_line(&path_str, &["--version"]) {
                py_item.version = Some(ver);
            }

            // Check Virtual Environment
            if std::env::var("VIRTUAL_ENV").is_ok() {
                py_item.details.push(format!(
                    "Active Virtual Environment: {}",
                    std::env::var("VIRTUAL_ENV").unwrap_or_default()
                ));
            } else if std::env::var("CONDA_PREFIX").is_ok() {
                py_item.details.push(format!(
                    "Active Conda Environment: {}",
                    std::env::var("CONDA_PREFIX").unwrap_or_default()
                ));
            } else {
                py_item.details.push("No virtual environment active (using system/global Python)".to_string());
            }

            result.items.push(py_item);

            // 2. Pip Check & Alignment
            let pip_path = find_executable("pip3").or_else(|| find_executable("pip"));
            if let Some(pip_p) = pip_path {
                let pip_str = pip_p.to_string_lossy().to_string();
                let mut pip_item = DiagnosticItem::ok("Pip Package Manager");
                pip_item.path = Some(pip_str.clone());

                if let Some(ver) = run_cmd_first_line(&pip_str, &["--version"]) {
                    pip_item.version = Some(ver.clone());
                    
                    // Verify pip points to current Python
                    // Output format is usually: pip X.Y.Z from /path (python X.Y)
                    if let Some(py_ver_out) = run_cmd_first_line(&path_str, &["-c", "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')"]) {
                        let expected_tag = format!("python {}", py_ver_out);
                        if !ver.contains(&expected_tag) && !ver.contains(&format!("python{}", py_ver_out)) {
                            pip_item.status = Status::Warning;
                            pip_item.issues.push(Issue::new(
                                Status::Warning,
                                "pip binary might not be aligned with the current python interpreter",
                            ));
                            pip_item.recommendations.push(Recommendation::full(
                                "Run pip via python module to avoid version mismatches",
                                format!("{} -m pip install <package>", path_str),
                                "This guarantees installing packages into the exact Python environment being run.",
                            ));
                        }
                    }
                }
                result.items.push(pip_item);
            } else {
                let mut pip_item = DiagnosticItem::warning(
                    "Pip Package Manager",
                    "pip / pip3 executable was not found on PATH",
                );
                pip_item.recommendations.push(Recommendation::with_command(
                    "Install pip using ensurepip",
                    format!("{} -m ensurepip --upgrade", path_str),
                ));
                result.items.push(pip_item);
            }

            // 3. Modern Package Tools (uv, poetry, pdm, conda)
            let mut tools = Vec::new();
            if let Some(p) = find_executable("uv") {
                if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                    tools.push(format!("uv ({})", v));
                }
            }
            if let Some(p) = find_executable("poetry") {
                if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                    tools.push(format!("poetry ({})", v));
                }
            }
            if let Some(p) = find_executable("conda") {
                if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                    tools.push(format!("conda ({})", v));
                }
            }
            if !tools.is_empty() {
                let mut tool_item = DiagnosticItem::ok("Python Project / Package Tools");
                tool_item.details = tools;
                result.items.push(tool_item);
            }

            // 4. ML / Deep Learning Frameworks GPU Recognition (PyTorch / TensorFlow / JAX)
            let torch_check = run_cmd_first_line(
                &path_str,
                &[
                    "-c",
                    "import torch; print(f'PyTorch {torch.__version__} (CUDA available: {torch.cuda.is_available()})')",
                ],
            );
            if let Some(torch_info) = torch_check {
                let mut torch_item = DiagnosticItem::ok("PyTorch Deep Learning Runtime");
                torch_item.details.push(torch_info.clone());
                if torch_info.contains("CUDA available: False") {
                    torch_item.status = Status::Info;
                    torch_item.issues.push(Issue::new(
                        Status::Info,
                        "PyTorch is running in CPU-only mode (CUDA is not available)",
                    ));
                    torch_item.recommendations.push(Recommendation::full(
                        "Install PyTorch with CUDA support if you have an NVIDIA GPU",
                        "pip install torch torchvision --index-url https://download.pytorch.org/whl/cu121",
                        "Refer to pytorch.org for official build matrix.",
                    ));
                }
                result.items.push(torch_item);
            }
        } else {
            let mut py_item = DiagnosticItem::error(
                "Python Interpreter",
                "Neither 'python3' nor 'python' was found on your PATH",
            );
            let install_cmd = crate::system::package_manager::get_install_command("python3")
                .unwrap_or_else(|| "sudo apt install python3 python3-pip".to_string());
            py_item.recommendations.push(Recommendation::full(
                "Install Python 3",
                install_cmd,
                "Python 3 is required for Python development.",
            ));
            result.items.push(py_item);
        }

        result
    }
}
