//! Health check and version info example.
//!
//! Demonstrates connecting to Docling Serve and checking its health
//! and version status. This is the simplest possible use of the SDK
//! and a good way to verify your connection is working.
//!
//! # Usage
//! ```sh
//! cargo run --example health
//! ```

use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    // Health check — verifies the server is running and responsive.
    let health = client.health().await?;
    println!("Health status: {}", health.status);

    // Version info — returns server and component versions.
    let version = client.version().await?;
    println!("Version info:");
    for (key, value) in &version {
        println!("  {}: {}", key, value);
    }

    Ok(())
}
