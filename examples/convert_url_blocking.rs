//! Synchronous document conversion example.
//!
//! Demonstrates using the blocking API to convert documents without
//! needing async/await. Useful for simple scripts and CLI tools.
//!
//! # Usage
//! ```sh
//! cargo run --example convert_url_blocking
//! ```

use docling_rs::blocking::DoclingClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client (no tokio::main needed!)
    let client = DoclingClient::new("http://127.0.0.1:5001");

    // Health check
    let health = client.health()?;
    println!("Server status: {}", health.status);

    // Convert a URL to Markdown
    println!("\nConverting document...");
    let result = client.convert_source("https://arxiv.org/pdf/2206.01062", None)?;

    println!("Status: {:?}", result.status);

    if let Some(md) = &result.document.md_content {
        println!("\nMarkdown output (first 500 chars):");
        println!("{}", &md[..md.len().min(500)]);
    }

    Ok(())
}
