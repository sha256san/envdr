use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "envdoctor",
    about = "🩺 Automated Developer Environment Diagnostic & Health Check Tool",
    version,
    long_about = "envdoctor (envdr) diagnoses your programming languages (Python, Rust, Go, Node.js, Java, Ruby, PHP, C#, C++, Swift, etc.) and developer tools (Docker, GPU/CUDA, Git, Shell), identifies root causes of issues, and suggests actionable fixes."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Check programming language environment(s) (e.g. --language python, or -l for all installed)
    #[arg(short = 'l', long = "language", value_name = "LANG", num_args = 0..=1, default_missing_value = "all", global = true)]
    pub language: Option<String>,

    /// Check developer tool(s) (e.g. --tool docker, or -t for all tools)
    #[arg(short = 't', long = "tool", value_name = "TOOL", num_args = 0..=1, default_missing_value = "all", global = true)]
    pub tool: Option<String>,

    // Direct language flags
    /// Diagnose Python environment
    #[arg(long, global = true)]
    pub python: bool,

    /// Diagnose Rust toolchain
    #[arg(long, global = true)]
    pub rust: bool,

    /// Diagnose Node.js / JavaScript / TypeScript runtime
    #[arg(long, global = true)]
    pub node: bool,

    /// Diagnose Go toolchain
    #[arg(long, global = true)]
    pub go: bool,

    /// Diagnose C / C++ toolchain
    #[arg(long, global = true)]
    pub cpp: bool,

    /// Diagnose Java / JVM environment
    #[arg(long, global = true)]
    pub java: bool,

    /// Diagnose Ruby environment
    #[arg(long, global = true)]
    pub ruby: bool,

    /// Diagnose PHP environment
    #[arg(long, global = true)]
    pub php: bool,

    /// Diagnose .NET / C# environment
    #[arg(long, global = true)]
    pub dotnet: bool,

    /// Diagnose Swift toolchain
    #[arg(long, global = true)]
    pub swift: bool,

    /// Diagnose Dart / Flutter SDK
    #[arg(long, global = true)]
    pub dart: bool,

    /// Diagnose Zig compiler
    #[arg(long, global = true)]
    pub zig: bool,

    /// Diagnose Lua runtime
    #[arg(long, global = true)]
    pub lua: bool,

    // Direct tool flags
    /// Diagnose Docker & Containerization
    #[arg(long, global = true)]
    pub docker: bool,

    /// Diagnose GPU & Hardware Acceleration (CUDA / ROCm)
    #[arg(long, global = true)]
    pub gpu: bool,

    /// Diagnose Git & Version Control
    #[arg(long, global = true)]
    pub git: bool,

    /// Diagnose Shell & Configuration
    #[arg(long, global = true)]
    pub shell: bool,

    /// Diagnose System & Hardware Resources
    #[arg(long, global = true)]
    pub system: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Terminal, global = true)]
    pub format: OutputFormat,

    /// Show detailed diagnosis information
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Only show warnings and errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Write report to a file instead of stdout
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,

    /// Preview or apply automatic fixes for detected issues
    #[arg(long, global = true)]
    pub fix: bool,

    /// Apply fixes directly (when using --fix or fix subcommand)
    #[arg(long, global = true)]
    pub apply: bool,

    /// Preview fix plan without applying changes (default)
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Run in CI mode (fail with exit code on any warnings or errors)
    #[arg(long, global = true)]
    pub ci: bool,
}

impl Cli {
    /// 直接指定されたフラグ群（--python, --docker 等）をターゲットリストとして抽出
    pub fn direct_flag_targets(&self) -> Vec<String> {
        let mut targets = Vec::new();

        if self.python { targets.push("python".to_string()); }
        if self.rust { targets.push("rust".to_string()); }
        if self.node { targets.push("node".to_string()); }
        if self.go { targets.push("go".to_string()); }
        if self.cpp { targets.push("cpp".to_string()); }
        if self.java { targets.push("java".to_string()); }
        if self.ruby { targets.push("ruby".to_string()); }
        if self.php { targets.push("php".to_string()); }
        if self.dotnet { targets.push("dotnet".to_string()); }
        if self.swift { targets.push("swift".to_string()); }
        if self.dart { targets.push("dart".to_string()); }
        if self.zig { targets.push("zig".to_string()); }
        if self.lua { targets.push("lua".to_string()); }

        if self.docker { targets.push("docker".to_string()); }
        if self.gpu { targets.push("gpu".to_string()); }
        if self.git { targets.push("git".to_string()); }
        if self.shell { targets.push("shell".to_string()); }
        if self.system { targets.push("system".to_string()); }

        targets
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a complete diagnostic across all supported environments (default)
    Doctor,

    /// Run diagnostic for a specific target environment
    Check {
        /// Target category to diagnose (system, shell, python, rust, go, node, java, ruby, php, dotnet, cpp, docker, gpu, git, etc.)
        #[arg(value_name = "TARGET")]
        target: String,
    },

    /// Generate an environment report
    Report,

    /// Safely fix or generate fix commands for detected issues
    Fix {
        /// Target category to fix (optional)
        #[arg(value_name = "TARGET")]
        target: Option<String>,

        /// Apply fixes directly (default is dry-run)
        #[arg(long, default_value_t = false)]
        apply: bool,

        /// Dry-run mode (display planned fix commands only)
        #[arg(long, default_value_t = true)]
        dry_run: bool,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Terminal,
    Json,
    Markdown,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Terminal => write!(f, "terminal"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Markdown => write!(f, "markdown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_flags_parsing() {
        let cli = Cli::try_parse_from(["envdoctor", "--python", "--rust"]).unwrap();
        assert!(cli.python);
        assert!(cli.rust);
        assert!(!cli.go);
        assert_eq!(cli.direct_flag_targets(), vec!["python", "rust"]);
    }

    #[test]
    fn test_cli_language_option() {
        let cli = Cli::try_parse_from(["envdoctor", "--language", "go,python"]).unwrap();
        assert_eq!(cli.language, Some("go,python".to_string()));

        let cli_bare = Cli::try_parse_from(["envdoctor", "--language"]).unwrap();
        assert_eq!(cli_bare.language, Some("all".to_string()));
    }

    #[test]
    fn test_cli_tool_option() {
        let cli = Cli::try_parse_from(["envdoctor", "--tool", "docker,gpu"]).unwrap();
        assert_eq!(cli.tool, Some("docker,gpu".to_string()));

        let cli_bare = Cli::try_parse_from(["envdoctor", "--tool"]).unwrap();
        assert_eq!(cli_bare.tool, Some("all".to_string()));
    }
}
