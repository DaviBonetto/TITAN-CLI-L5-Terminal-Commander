//! Status command implementation
//!
//! Checks the health of all Titan Protocol services and displays
//! a formatted status table with connection indicators.

use crate::utils::{client::TitanClient, ui};
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Service definition with name, endpoint, and icon
struct Service {
    name: &'static str,
    icon: &'static str,
    endpoint: &'static str,
    description: &'static str,
}

/// List of all Titan Protocol services to check
const SERVICES: &[Service] = &[
    Service {
        name: "CERBERUS",
        icon: "🛡️",
        endpoint: "http://localhost:8080/health",
        description: "API Gateway (L2)",
    },
    Service {
        name: "KRONOS",
        icon: "⏰",
        endpoint: "http://localhost:3000/health",
        description: "Task Scheduler",
    },
    Service {
        name: "HERMES",
        icon: "📨",
        endpoint: "http://localhost:50051",
        description: "Event Bus (gRPC)",
    },
    Service {
        name: "VORTEX",
        icon: "🧠",
        endpoint: "http://localhost:8000/health",
        description: "AI Engine",
    },
    Service {
        name: "OPTICUS",
        icon: "👁️",
        endpoint: "http://localhost:8100/health",
        description: "Vision Pipeline",
    },
];

/// Execute the status command
pub async fn execute(service_filter: Option<String>, detailed: bool, verbose: bool) -> Result<()> {
    // Print header
    println!();
    ui::print_header("TITAN PROTOCOL STATUS");
    println!();

    // Create HTTP client
    let client = TitanClient::new()?;

    // Create progress bar for scanning
    let pb = ProgressBar::new(SERVICES.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message("Scanning services...");

    // Check each service
    let mut results = Vec::new();

    for service in SERVICES {
        // Apply filter if specified
        if let Some(ref filter) = service_filter {
            if !service.name.to_lowercase().contains(&filter.to_lowercase()) {
                continue;
            }
        }

        pb.set_message(format!("Checking {}...", service.name));

        let status = client.check_health(service.endpoint).await;
        results.push((service, status));

        pb.inc(1);
    }

    pb.finish_and_clear();

    // Print results table
    print_status_table(&results, detailed, verbose);

    // Print summary
    let online = results.iter().filter(|(_, s)| s.is_ok()).count();
    let total = results.len();

    println!();
    if online == total {
        println!(
            "  {} All {} services operational",
            "✓".green().bold(),
            total
        );
    } else {
        println!(
            "  {} {}/{} services online",
            "⚠".yellow().bold(),
            online,
            total
        );
    }
    println!();

    Ok(())
}

/// Print the status table
fn print_status_table(
    results: &[(&Service, Result<String, reqwest::Error>)],
    detailed: bool,
    _verbose: bool,
) {
    // Table header
    println!(
        "  {} {} {} {}",
        "SERVICE".cyan().bold(),
        " ".repeat(10),
        "STATUS".cyan().bold(),
        if detailed {
            "       DETAILS".cyan().bold().to_string()
        } else {
            "".to_string()
        }
    );
    println!("  {}", "─".repeat(60));

    for (service, status) in results {
        let (status_text, status_color) = match status {
            Ok(msg) => {
                let display = format!("[ONLINE]  {}", "●".green());
                (display, msg.clone())
            }
            Err(_) => {
                let display = format!("[OFFLINE] {}", "○".red());
                (display, "Connection refused".to_string())
            }
        };

        // Service name with icon
        let name_display = format!("{} {}", service.icon, service.name);
        let padding = 16 - name_display.chars().count();

        print!("  {}{}", name_display, " ".repeat(padding.max(1)));

        if status.is_ok() {
            print!("{}", status_text.green());
        } else {
            print!("{}", status_text.red());
        }

        if detailed {
            print!("  {}", service.description.dimmed());
            if status.is_err() {
                print!(" - {}", status_color.red().dimmed());
            }
        }

        println!();
    }

    println!("  {}", "─".repeat(60));
}
