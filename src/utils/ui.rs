//! UI utilities for TITAN-CLI
//! 
//! Provides consistent styling and formatting for terminal output.

use colored::Colorize;

/// Print a styled header
pub fn print_header(title: &str) {
    let width = 60;
    let title_len = title.len();
    let padding = (width - title_len - 4) / 2;

    println!("  {}", "╔".cyan().to_string() + &"═".repeat(width - 2).cyan().to_string() + &"╗".cyan().to_string());
    println!(
        "  {} {} {} {}",
        "║".cyan(),
        " ".repeat(padding),
        title.white().bold(),
        " ".repeat(width - padding - title_len - 4) + &"║".cyan().to_string()
    );
    println!("  {}", "╚".cyan().to_string() + &"═".repeat(width - 2).cyan().to_string() + &"╝".cyan().to_string());
}

/// Print a success message
#[allow(dead_code)]
pub fn print_success(message: &str) {
    println!("  {} {}", "✓".green().bold(), message.green());
}

/// Print an error message
#[allow(dead_code)]
pub fn print_error(message: &str) {
    println!("  {} {}", "✖".red().bold(), message.red());
}

/// Print a warning message
#[allow(dead_code)]
pub fn print_warning(message: &str) {
    println!("  {} {}", "⚠".yellow().bold(), message.yellow());
}

/// Print an info message
#[allow(dead_code)]
pub fn print_info(message: &str) {
    println!("  {} {}", "ℹ".cyan().bold(), message);
}

/// Print a key-value pair
#[allow(dead_code)]
pub fn print_kv(key: &str, value: &str) {
    println!("  {} {}", format!("{}:", key).dimmed(), value);
}

/// Print a divider line
#[allow(dead_code)]
pub fn print_divider() {
    println!("  {}", "─".repeat(60).dimmed());
}

/// Get a colored status indicator
#[allow(dead_code)]
pub fn status_indicator(online: bool) -> String {
    if online {
        "●".green().to_string()
    } else {
        "○".red().to_string()
    }
}
