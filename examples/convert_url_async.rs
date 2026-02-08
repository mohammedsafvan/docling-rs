//! Asynchronous URL-based document conversion with polling.
//!
//! Demonstrates the async conversion workflow:
//! 1. Submit a document URL for background processing.
//! 2. Poll the server for task status until completion.
//! 3. Retrieve the converted result.
//!
//! Also shows the convenience `wait_for_conversion` method that wraps
//! all three steps into a single call.
//!
//! # Usage
//! ```sh
//! cargo run --example convert_url_async
//! ```

use std::time::Duration;

use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    // --- Convenience method: submit + poll + fetch in one call ---
    println!("=== Async conversion (wait_for_conversion) ===\n");

    let result = client
        .wait_for_conversion(
            "https://arxiv.org/pdf/2206.01062",
            None,                          // server defaults
            Duration::from_secs(300),      // timeout
            Some(5.0),                     // poll every 5 seconds
        )
        .await?;

    println!("Status: {:?}", result.status);
    println!("Processing time: {:.2}s", result.processing_time);

    if let Some(md) = &result.document.md_content {
        println!(
            "Markdown (first 500 chars):\n{}\n",
            &md[..md.len().min(500)]
        );
    }

    // --- Manual async flow: submit, poll, fetch separately ---
    println!("=== Manual async flow ===\n");

    // Step 1: Submit the task
    let task = client
        .convert_source_async("https://arxiv.org/pdf/2206.01062", None)
        .await?;
    println!("Task submitted: {}", task.task_id);
    println!("Initial status: {}", task.task_status);

    // Step 2: Poll until done (with long-polling)
    loop {
        let status = client
            .poll_task_status(&task.task_id, Some(5.0))
            .await?;

        println!(
            "  Polling... status={}, position={:?}",
            status.task_status, status.task_position
        );

        if let Some(ref meta) = status.task_meta {
            println!(
                "    Progress: {}/{} processed, {} succeeded, {} failed",
                meta.num_processed, meta.num_docs, meta.num_succeeded, meta.num_failed
            );
        }

        match status.task_status.as_str() {
            "SUCCESS" => break,
            "FAILURE" => {
                eprintln!("Task failed!");
                return Ok(());
            }
            _ => continue,
        }
    }

    // Step 3: Fetch the result
    let result = client.get_task_result(&task.task_id).await?;
    println!("\nFinal status: {:?}", result.status);
    println!("Processing time: {:.2}s", result.processing_time);

    Ok(())
}
