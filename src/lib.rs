pub mod checkers;
pub mod cli;
pub mod core;
pub mod output;
pub mod system;
pub mod utils;

use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use core::{CategoryResult, DiagnosticSummary, FullDiagnosticReport};
use output::{JsonFormatter, MarkdownFormatter, TerminalFormatter};
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let registry = checkers::CheckerRegistry::default();

    // Check if top-level --fix flag or `fix` subcommand was requested
    if cli.fix || matches!(&cli.command, Some(Commands::Fix { .. })) {
        let (target, apply) = match &cli.command {
            Some(Commands::Fix { target, apply, .. }) => (target.as_deref(), *apply || cli.apply),
            _ => (None, cli.apply),
        };

        let all_results = registry.run_all();
        let fixes = core::fixer::AutoFixer::plan_fixes(&all_results, target);
        core::fixer::AutoFixer::execute_plan(&fixes, apply)?;

        // If applied, automatically run a re-scan to show resolved status
        if apply && !fixes.is_empty() {
            println!("\n🔍 Running automatic post-fix re-scan...\n");
            let after_results = registry.run_all();
            let after_summary = DiagnosticSummary::from_categories(&after_results);
            let sys_report = system::SystemReport::collect();
            let full_report = FullDiagnosticReport {
                version: env!("CARGO_PKG_VERSION").to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                system: sys_report,
                summary: after_summary,
                results: after_results,
            };
            let formatter = TerminalFormatter::new(false, false);
            formatter.print_summary(&full_report);
        }

        return Ok(());
    }

    let results: Vec<CategoryResult> = match &cli.command {
        Some(Commands::Check { target }) => {
            if let Some(res) = registry.run_target(target) {
                vec![res]
            } else {
                eprintln!(
                    "Error: Unknown diagnostic target '{}'. Available targets: {}",
                    target,
                    registry.available_targets().join(", ")
                );
                std::process::exit(11); // Invalid option / target
            }
        }
        None | Some(Commands::Doctor) | Some(Commands::Report) => {
            let direct_targets = cli.direct_flag_targets();
            let has_direct_flags = !direct_targets.is_empty();
            let has_language_opt = cli.language.is_some();
            let has_tool_opt = cli.tool.is_some();

            if has_direct_flags || has_language_opt || has_tool_opt {
                let mut combined_results = Vec::new();
                let mut executed_categories = std::collections::HashSet::new();

                // 1. Direct flags (--python, --docker, etc.)
                if has_direct_flags {
                    for res in registry.run_multiple_targets(&direct_targets) {
                        if executed_categories.insert(res.category.clone()) {
                            combined_results.push(res);
                        }
                    }
                }

                // 2. Language options (--language [LANG])
                if let Some(ref lang_arg) = cli.language {
                    let targets: Vec<String> = if lang_arg.eq_ignore_ascii_case("all") {
                        Vec::new()
                    } else {
                        lang_arg.split(',').map(|s| s.trim().to_string()).collect()
                    };

                    let lang_results = if targets.is_empty() {
                        registry.run_languages(None)
                    } else {
                        registry.run_languages(Some(&targets))
                    };

                    for res in lang_results {
                        if executed_categories.insert(res.category.clone()) {
                            combined_results.push(res);
                        }
                    }
                }

                // 3. Tool options (--tool [TOOL])
                if let Some(ref tool_arg) = cli.tool {
                    let targets: Vec<String> = if tool_arg.eq_ignore_ascii_case("all") {
                        Vec::new()
                    } else {
                        tool_arg.split(',').map(|s| s.trim().to_string()).collect()
                    };

                    let tool_results = if targets.is_empty() {
                        registry.run_tools(None)
                    } else {
                        registry.run_tools(Some(&targets))
                    };

                    for res in tool_results {
                        if executed_categories.insert(res.category.clone()) {
                            combined_results.push(res);
                        }
                    }
                }

                combined_results
            } else {
                // デフォルト: インストール済み言語 + システム + ツール全体診断
                registry.run_all()
            }
        }
        Some(Commands::Fix { .. }) => unreachable!(),
    };

    let sys_report = system::SystemReport::collect();
    let summary = DiagnosticSummary::from_categories(&results);
    let now = chrono::Utc::now().to_rfc3339();

    let full_report = FullDiagnosticReport {
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: now,
        system: sys_report,
        summary,
        results,
    };

    let output_text = match cli.format {
        OutputFormat::Terminal => {
            let formatter = TerminalFormatter::new(cli.verbose, cli.quiet);
            formatter.print_report(&full_report);
            None
        }
        OutputFormat::Json => {
            let json = JsonFormatter::format(&full_report)?;
            Some(json)
        }
        OutputFormat::Markdown => {
            let md = MarkdownFormatter::format(&full_report);
            Some(md)
        }
    };

    if let Some(text) = output_text {
        if let Some(ref out_file) = cli.output {
            fs::write(out_file, &text)?;
            println!("Report written to {}", out_file.display());
        } else {
            println!("{}", text);
        }
    }

    // Determine exit codes according to specification (addplan4 §14)
    if full_report.summary.critical > 0 {
        std::process::exit(3);
    } else if full_report.summary.error > 0 {
        std::process::exit(2);
    } else if full_report.summary.warning > 0 && cli.ci {
        std::process::exit(1);
    }

    Ok(())
}
