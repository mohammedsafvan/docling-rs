//! Asynchronous local file conversion with polling.
//!
//! Demonstrates uploading local files for background processing,
//! then polling for completion. Useful for large files or batch
//! processing where you don't want to block on a single HTTP request.
//!
//! # Usage
//! ```sh
//! cargo run --example convert_file_async
//! ```

use std::time::Duration;

use docling_rs::models::requests::ConvertDocumentsRequestOptions;
use docling_rs::{DoclingClient, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    // --- Convenience method: submit + poll + fetch ---
    println!("=== Async file conversion (wait_for_file_conversion) ===\n");

    let opts = ConvertDocumentsRequestOptions {
        to_formats: Some(vec![OutputFormat::Md, OutputFormat::Json]),
        do_ocr: Some(true),
        ..Default::default()
    };

    let result = client
        .wait_for_file_conversion(
            &["./document.pdf"],
            Some(&opts),
            None,                          // default target (in-body)
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

    // --- Manual async flow for files ---
    println!("=== Manual async file flow ===\n");

    // Step 1: Submit
    let task = client
        .convert_file_async(&["./document.pdf"], None, None)
        .await?;
    println!("Task submitted: {}", task.task_id);

    // Step 2: Poll
    loop {
        let status = client
            .poll_task_status(&task.task_id, Some(5.0))
            .await?;

        println!("  Status: {}", status.task_status);

        if let Some(ref meta) = status.task_meta {
            println!(
                "    Progress: {}/{} docs processed",
                meta.num_processed, meta.num_docs
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

    // Step 3: Fetch result
    let result = client.get_task_result(&task.task_id).await?;
    println!("\nFinal status: {:?}", result.status);
    println!("Processing time: {:.2}s", result.processing_time);

    Ok(())
}
