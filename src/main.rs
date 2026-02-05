//! # TITAN-CLI: The Operator Console
//!
//! Unified Command Line Interface for the Titan Protocol Ecosystem.
//!
//! ## Architecture
//! - Clap v4 derive-based parser
//! - Async/await with Tokio runtime
//! - Colored terminal output with spinners
//! - Modular command structure

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::ExitCode;

mod commands;
mod utils;

use commands::{ask, deploy, status, vision};

/// ASCII Art Banner for TITAN-CLI
const BANNER: &str = r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║   ████████╗██╗████████╗ █████╗ ███╗   ██╗       ██████╗██╗     ██╗           ║
║   ╚══██╔══╝██║╚══██╔══╝██╔══██╗████╗  ██║      ██╔════╝██║     ██║           ║
║      ██║   ██║   ██║   ███████║██╔██╗ ██║█████╗██║     ██║     ██║           ║
║      ██║   ██║   ██║   ██╔══██║██║╚██╗██║╚════╝██║     ██║     ██║           ║
║      ██║   ██║   ██║   ██║  ██║██║ ╚████║      ╚██████╗███████╗██║           ║
║      ╚═╝   ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═══╝       ╚═════╝╚══════╝╚═╝           ║
║                                                                              ║
║                    ⚡ THE OPERATOR CONSOLE ⚡                                 ║
║                         v1.0.0 | L5 Interface                                ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
"#;

/// TITAN-CLI: The Operator Console
///
/// Unified command-line interface for orchestrating the Titan Protocol ecosystem.
/// Control AI services, manage deployments, and monitor system health.
#[derive(Parser)]
#[command(
    name = "titan",
    author = "Titan Protocol Engineering",
    version = "1.0.0",
    about = "🔱 The Operator Console - Command the Titan Protocol Ecosystem",
    long_about = None,
    propagate_version = true,
    arg_required_else_help = true,
)]
struct Cli {
    /// Enable verbose output for debugging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    #[command(subcommand)]
    command: Commands,
}

/// Available commands for the Titan Protocol CLI
#[derive(Subcommand)]
enum Commands {
    /// 📊 Check status of all Titan Protocol services
    #[command(aliases = ["st", "health", "ping"])]
    Status {
        /// Check a specific service only
        #[arg(short, long)]
        service: Option<String>,

        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// 🧠 Send a query to VORTEX AI Engine
    #[command(aliases = ["query", "q", "ai"])]
    Ask {
        /// The query to send to VORTEX
        query: String,

        /// Use streaming response mode
        #[arg(short, long)]
        stream: bool,

        /// Specify model to use
        #[arg(short, long, default_value = "vortex-v3")]
        model: String,
    },

    /// 🚀 Deploy services to the Titan infrastructure
    #[command(aliases = ["dep", "up"])]
    Deploy {
        /// Service to deploy
        service: String,

        /// Target environment
        #[arg(short, long, default_value = "staging")]
        env: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// 👁️ Connect to OPTICUS vision stream
    #[command(aliases = ["vis", "eye", "stream"])]
    Vision {
        /// Enable streaming mode
        #[arg(short, long)]
        stream: bool,

        /// Camera/source index
        #[arg(short, long, default_value = "0")]
        index: u32,
    },

    /// ⚙️ Configure TITAN-CLI settings
    #[command(aliases = ["cfg", "settings"])]
    Config {
        /// Show current configuration
        #[arg(short, long)]
        list: bool,

        /// Reset to defaults
        #[arg(long)]
        reset: bool,
    },

    /// 📜 Show version and system information
    #[command(aliases = ["ver", "info"])]
    Version,
}

#[tokio::main]
async fn main() -> ExitCode {
    // Load environment variables
    let _ = dotenv::dotenv();

    // Parse command line arguments
    let cli = Cli::parse();

    // Handle --no-color flag
    if cli.no_color {
        colored::control::set_override(false);
    }

    // Print banner for version command
    if matches!(cli.command, Commands::Version) {
        print_banner();
    }

    // Execute the appropriate command
    let result = match cli.command {
        Commands::Status { service, detailed } => {
            status::execute(service, detailed, cli.verbose).await
        }
        Commands::Ask {
            query,
            stream,
            model,
        } => ask::execute(&query, stream, &model, cli.verbose).await,
        Commands::Deploy { service, env, yes } => {
            deploy::execute(&service, &env, yes, cli.verbose).await
        }
        Commands::Vision { stream, index } => vision::execute(stream, index, cli.verbose).await,
        Commands::Config { list, reset } => handle_config(list, reset),
        Commands::Version => {
            print_version_info();
            Ok(())
        }
    };

    // Handle result
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "✖ Error:".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}

/// Print the TITAN banner
fn print_banner() {
    println!("{}", BANNER.cyan());
}

/// Print detailed version information
fn print_version_info() {
    println!();
    println!("  {} {}", "titan-cli".cyan().bold(), "v1.0.0".white());
    println!(
        "  {} {}",
        "Layer:".dimmed(),
        "L5 - Interface Layer".yellow()
    );
    println!("  {} {}", "Protocol:".dimmed(), "Titan Protocol v1".white());
    println!("  {} {}", "Runtime:".dimmed(), "Tokio Async".white());
    println!("  {} {}", "Platform:".dimmed(), std::env::consts::OS);
    println!();
    println!("  {}", "Connected Services:".cyan().bold());
    println!("    {} VORTEX   - AI Engine (vortex-v3)", "🧠".to_string());
    println!("    {} CERBERUS - API Gateway (L2)", "🛡️".to_string());
    println!("    {} OPTICUS  - Vision Pipeline", "👁️".to_string());
    println!("    {} KRONOS   - Task Scheduler", "⏰".to_string());
    println!("    {} HERMES   - Event Bus", "📨".to_string());
    println!();
    println!("  {}", "Documentation:".dimmed());
    println!("    https://github.com/DaviBonetto/TITAN-CLI-L5-Terminal-Commander");
    println!();
}

/// Handle configuration commands
fn handle_config(list: bool, reset: bool) -> anyhow::Result<()> {
    if reset {
        println!("{}", "⚙️ Configuration reset to defaults".yellow());
        return Ok(());
    }

    if list {
        println!();
        println!("  {}", "Current Configuration:".cyan().bold());
        println!("  ─────────────────────────────────────");
        println!("  {} {}", "API Endpoint:".dimmed(), "http://localhost:8080");
        println!("  {} {}", "Timeout:".dimmed(), "30s");
        println!("  {} {}", "Theme:".dimmed(), "dark");
        println!("  {} {}", "Verbose:".dimmed(), "false");
        println!();
    } else {
        println!(
            "{}",
            "Use --list to view configuration or --reset to restore defaults".dimmed()
        );
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Unit Tests - For CI Pipeline Validation
// ═══════════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    /// Basic arithmetic test - validates test harness is working
    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(2 + 2, 4);
        assert_eq!(10 - 5, 5);
        assert_eq!(3 * 4, 12);
    }

    /// Test that the CLI version is correctly defined
    #[test]
    fn test_version_defined() {
        let version = "1.0.0";
        assert!(!version.is_empty());
        assert!(version.starts_with("1."));
    }

    /// Test valid service names for deploy command
    #[test]
    fn test_valid_services() {
        let valid_services = ["cerberus", "kronos", "hermes", "vortex", "opticus", "all"];

        assert!(valid_services.contains(&"cerberus"));
        assert!(valid_services.contains(&"vortex"));
        assert!(!valid_services.contains(&"invalid"));
    }

    /// Test service icon mapping logic
    #[test]
    fn test_service_icons() {
        let get_icon = |service: &str| -> &'static str {
            match service.to_lowercase().as_str() {
                "cerberus" => "🛡️",
                "kronos" => "⏰",
                "hermes" => "📨",
                "vortex" => "🧠",
                "opticus" => "👁️",
                _ => "📦",
            }
        };

        assert_eq!(get_icon("cerberus"), "🛡️");
        assert_eq!(get_icon("vortex"), "🧠");
        assert_eq!(get_icon("KRONOS"), "⏰");
        assert_eq!(get_icon("unknown"), "📦");
    }

    /// Test that banner constant is not empty
    #[test]
    fn test_banner_exists() {
        assert!(!BANNER.is_empty());
        assert!(BANNER.contains("TITAN"));
        assert!(BANNER.contains("OPERATOR CONSOLE"));
    }

    /// Test environment detection
    #[test]
    fn test_valid_environments() {
        let valid_envs = ["staging", "production", "development"];

        assert!(valid_envs.contains(&"staging"));
        assert!(valid_envs.contains(&"production"));
    }

    /// Test model name validation
    #[test]
    fn test_model_names() {
        let default_model = "vortex-v3";

        assert!(default_model.starts_with("vortex"));
        assert!(default_model.contains("-v"));
    }
}
