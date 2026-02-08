//! Synchronous local file conversion via multipart upload.
//!
//! Demonstrates converting a local file by uploading it to Docling Serve
//! via multipart/form-data. The call blocks until conversion is complete.
//! Also shows how to configure options and target type for file uploads.
//!
//! # Usage
//! ```sh
//! cargo run --example convert_file
//! ```

use docling_rs::models::requests::ConvertDocumentsRequestOptions;
use docling_rs::{DoclingClient, OutputFormat, TargetName};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    // --- Basic file upload (server defaults) ---
    println!("=== Basic file upload ===\n");

    let result = client
        .convert_file(&["./document.pdf"], None, None)
        .await?;

    println!("Status: {:?}", result.status);
    println!("Processing time: {:.2}s", result.processing_time);

    if let Some(md) = &result.document.md_content {
        println!(
            "Markdown (first 500 chars):\n{}\n",
            &md[..md.len().min(500)]
        );
    }

    // --- File upload with options ---
    println!("=== File upload with options ===\n");

    let opts = ConvertDocumentsRequestOptions {
        to_formats: Some(vec![OutputFormat::Md, OutputFormat::Json]),
        do_ocr: Some(true),
        ..Default::default()
    };

    let result = client
        .convert_file(
            &["./document.pdf"],
            Some(&opts),
            Some(&TargetName::Inbody),
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

    if result.document.json_content.is_some() {
        println!("JSON content: (present, structured DoclingDocument)");
    }

    // --- Multiple files at once ---
    println!("=== Multiple file upload ===\n");

    let result = client
        .convert_file(
            &["./document.pdf", "./report.docx"],
            None,
            None,
        )
        .await?;

    println!("Status: {:?}", result.status);
    println!("Filename: {}", result.document.filename);

    Ok(())
}
