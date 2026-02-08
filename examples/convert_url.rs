//! Synchronous URL-based document conversion.
//!
//! Demonstrates converting a document from a URL using the synchronous API.
//! The call blocks until conversion is complete and returns the result directly.
//! Also shows how to configure conversion options (output formats, OCR, etc.).
//!
//! # Usage
//! ```sh
//! cargo run --example convert_url
//! ```

use docling_rs::models::requests::ConvertDocumentsRequestOptions;
use docling_rs::{DoclingClient, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    // --- Basic conversion (server defaults) ---
    println!("=== Basic URL conversion ===\n");

    let result = client
        .convert_source("https://arxiv.org/pdf/2206.01062", None)
        .await?;

    println!("Status: {:?}", result.status);
    println!("Processing time: {:.2}s", result.processing_time);

    if let Some(md) = &result.document.md_content {
        println!(
            "Markdown (first 500 chars):\n{}\n",
            &md[..md.len().min(500)]
        );
    }

    // --- Conversion with custom options ---
    println!("=== Conversion with options ===\n");

    let opts = ConvertDocumentsRequestOptions {
        to_formats: Some(vec![OutputFormat::Md, OutputFormat::Text]),
        do_ocr: Some(true),
        ..Default::default()
    };

    let result = client
        .convert_source("https://arxiv.org/pdf/2206.01062", Some(opts))
        .await?;

    println!("Status: {:?}", result.status);

    if let Some(text) = &result.document.text_content {
        println!(
            "Plain text (first 500 chars):\n{}\n",
            &text[..text.len().min(500)]
        );
    }

    if let Some(md) = &result.document.md_content {
        println!(
            "Markdown (first 500 chars):\n{}",
            &md[..md.len().min(500)]
        );
    }

    Ok(())
}
