//! Ask command implementation
//!
//! Sends queries to the VORTEX AI Engine and displays
//! streaming or complete responses with beautiful formatting.

use crate::utils::ui;
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::time::sleep;

/// Execute the ask command
pub async fn execute(query: &str, stream: bool, model: &str, verbose: bool) -> Result<()> {
    println!();
    ui::print_header("VORTEX AI ENGINE");

    // Print query
    println!();
    println!("  {} {}", "Query:".cyan().bold(), query);
    println!("  {} {}", "Model:".dimmed(), model);
    if stream {
        println!("  {} {}", "Mode:".dimmed(), "Streaming".yellow());
    }
    println!();

    // Create thinking spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    spinner.enable_steady_tick(Duration::from_millis(80));

    // Simulate thinking phases
    let phases = [
        "Initializing neural pathways...",
        "Analyzing query semantics...",
        "Searching knowledge base...",
        "Synthesizing response...",
    ];

    for phase in phases {
        spinner.set_message(phase.to_string());
        sleep(Duration::from_millis(300)).await;
    }

    spinner.finish_and_clear();

    // Generate mock response based on query
    let response = generate_mock_response(query, model);

    // Print response with typewriter effect if streaming
    println!("  {}", "─".repeat(60));
    println!();

    if stream {
        print!("  {} ", "🧠".to_string());
        typewriter_print(&response).await;
    } else {
        println!("  {} {}", "🧠".to_string(), response);
    }

    println!();
    println!("  {}", "─".repeat(60));

    // Print metadata
    if verbose {
        println!();
        println!("  {}", "Response Metadata:".dimmed());
        println!("    {} {}", "Tokens:".dimmed(), "142");
        println!("    {} {}", "Latency:".dimmed(), "1.2s");
        println!("    {} {}", "Model:".dimmed(), model);
    }

    println!();

    Ok(())
}

/// Generate a mock response based on the query
fn generate_mock_response(query: &str, model: &str) -> String {
    let query_lower = query.to_lowercase();

    if query_lower.contains("analyze") || query_lower.contains("sector") {
        format!(
            "{}

Based on my analysis using {}, I've identified the following patterns:

  {} Temporal variance detected in data streams
  {} 3 anomalous signal patterns require attention  
  {} Recommended action: Deploy monitoring probes

This analysis draws from the combined knowledge of the Titan Protocol
ecosystem. Shall I elaborate on any specific finding?",
            "Analysis Complete.".green().bold(),
            model.cyan(),
            "•".cyan(),
            "•".cyan(),
            "•".cyan()
        )
    } else if query_lower.contains("status") || query_lower.contains("health") {
        "All Titan Protocol subsystems are operating within normal parameters.
The CERBERUS gateway is processing requests efficiently, and HERMES
event throughput remains optimal."
            .to_string()
    } else if query_lower.contains("deploy") {
        "To deploy services, use the `titan deploy <service>` command.
Ensure you have proper credentials configured in your environment.
Current deployment targets: staging, production, edge."
            .to_string()
    } else {
        format!(
            "{}

I've processed your query through the {} neural architecture.
This feature is connected to VORTEX v3 - the Titan Protocol AI Engine.

For more specific assistance, try:
  {} titan ask \"analyze sector 7\"
  {} titan status --detailed
  {} titan vision --stream",
            "Query Received.".green().bold(),
            model.cyan(),
            "$".dimmed(),
            "$".dimmed(),
            "$".dimmed()
        )
    }
}

/// Print text with typewriter effect
async fn typewriter_print(text: &str) {
    for ch in text.chars() {
        print!("{}", ch);
        // Flush stdout to ensure immediate display
        use std::io::{self, Write};
        let _ = io::stdout().flush();

        // Variable delay for natural feel
        let delay = match ch {
            '.' | '!' | '?' | '\n' => 50,
            ',' | ':' | ';' => 30,
            ' ' => 10,
            _ => 5,
        };
        sleep(Duration::from_millis(delay)).await;
    }
    println!();
}
