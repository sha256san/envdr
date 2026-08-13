use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Issue, Recommendation, Status};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;
use std::path::Path;

pub struct JavaChecker;

impl Checker for JavaChecker {
    fn id(&self) -> &'static str {
        "java"
    }

    fn title(&self) -> &'static str {
        "Java / JVM Development Environment"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["jvm", "jdk", "kotlin", "scala"]
    }

    fn is_installed(&self) -> bool {
        find_executable("java").is_some() || find_executable("javac").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        // 1. Java Runtime (java)
        let java_path = find_executable("java");
        let javac_path = find_executable("javac");

        if java_path.is_some() || javac_path.is_some() {
            if let Some(ref p) = java_path {
                let path_str = p.to_string_lossy().to_string();
                let mut java_item = DiagnosticItem::ok("Java Runtime (JRE/JDK)");
                java_item.path = Some(path_str.clone());

                if let Some(v) = run_cmd_first_line(&path_str, &["-version"]) {
                    java_item.version = Some(v);
                } else if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                    java_item.version = Some(v);
                }

                result.items.push(java_item);
            }

            // 2. Java Compiler (javac)
            if let Some(ref p) = javac_path {
                let path_str = p.to_string_lossy().to_string();
                let mut javac_item = DiagnosticItem::ok("Java Compiler (javac)");
                javac_item.path = Some(path_str.clone());

                if let Some(v) = run_cmd_first_line(&path_str, &["-version"]) {
                    javac_item.version = Some(v);
                } else if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                    javac_item.version = Some(v);
                }

                result.items.push(javac_item);
            } else {
                let mut javac_item = DiagnosticItem::warning(
                    "Java Compiler (javac)",
                    "javac was not found. You may have only JRE installed instead of full JDK.",
                );
                javac_item.recommendations.push(Recommendation::full(
                    "Install full JDK (Java Development Kit)",
                    "sudo apt install default-jdk",
                    "JDK includes javac compiler needed for building Java projects.",
                ));
                result.items.push(javac_item);
            }

            // 3. JAVA_HOME Environment Variable
            let mut java_home_item = DiagnosticItem::ok("JAVA_HOME Environment Variable");
            match std::env::var("JAVA_HOME") {
                Ok(home) => {
                    java_home_item.path = Some(home.clone());
                    if Path::new(&home).exists() {
                        java_home_item.details.push(format!("Points to existing directory: {}", home));
                    } else {
                        java_home_item.status = Status::Warning;
                        java_home_item.issues.push(Issue::new(
                            Status::Warning,
                            format!("JAVA_HOME is set to '{}' but directory does not exist", home),
                        ));
                    }
                }
                Err(_) => {
                    java_home_item.status = Status::Info;
                    java_home_item.issues.push(Issue::new(
                        Status::Info,
                        "JAVA_HOME is not set in environment (many build tools recommend setting it)",
                    ));
                    if let Some(ref p) = java_path {
                        java_home_item.recommendations.push(Recommendation::with_command(
                            "Export JAVA_HOME in shell profile",
                            format!("export JAVA_HOME=$(dirname $(dirname $(readlink -f {})))", p.display()),
                        ));
                    }
                }
            }
            result.items.push(java_home_item);

            // 4. Build Tools (Maven, Gradle, Ant)
            let mut build_tools = Vec::new();
            if let Some(p) = find_executable("mvn") {
                if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                    build_tools.push(format!("Maven ({})", v));
                } else {
                    build_tools.push("Maven".to_string());
                }
            }
            if let Some(p) = find_executable("gradle") {
                if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["--version"]) {
                    build_tools.push(format!("Gradle ({})", v));
                } else {
                    build_tools.push("Gradle".to_string());
                }
            }
            if let Some(p) = find_executable("kotlinc") {
                if let Some(v) = run_cmd_first_line(&p.to_string_lossy(), &["-version"]) {
                    build_tools.push(format!("Kotlin Compiler ({})", v));
                }
            }

            if !build_tools.is_empty() {
                let mut tools_item = DiagnosticItem::ok("JVM Build Tools & Languages");
                tools_item.details = build_tools;
                result.items.push(tools_item);
            }

            // 5. Version Managers (SDKMAN, jEnv)
            let mut vms = Vec::new();
            if std::env::var("SDKMAN_DIR").is_ok() {
                vms.push("SDKMAN!".to_string());
            }
            if std::env::var("JENV_ROOT").is_ok() || find_executable("jenv").is_some() {
                vms.push("jEnv".to_string());
            }
            if !vms.is_empty() {
                let mut vm_item = DiagnosticItem::ok("Java Version Manager");
                vm_item.details = vms;
                result.items.push(vm_item);
            }
        } else {
            let mut java_item = DiagnosticItem::error(
                "Java / JDK",
                "Neither 'java' nor 'javac' was found on your PATH",
            );
            let install_cmd = crate::system::package_manager::get_install_command("java")
                .unwrap_or_else(|| "sudo apt install default-jdk".to_string());
            java_item.recommendations.push(Recommendation::full(
                "Install OpenJDK",
                install_cmd,
                "OpenJDK provides Java runtime and development compiler.",
            ));
            result.items.push(java_item);
        }

        result
    }
}
