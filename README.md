# docling_rs

Rust SDK for [Docling Serve](https://github.com/docling-project/docling-serve) that makes document conversion simple, reliable, and production-ready in Rust.

Convert PDFs, DOCX, PPTX, images, and more into Markdown, JSON, HTML, or plain text from Rust.

## Features

- **Synchronous & async conversion** — block until done, or submit and poll
- **Local file upload** — convert files from disk via multipart upload
- **Fully typed** — all enums, options, and responses match the OpenAPI 3.1 spec (v1.12.0)
- **Optional API key auth** — `Authorization: Bearer <key>` on all secured endpoints
- **Structured errors** — distinct variants for network, API, deserialization, file I/O, task failure, and timeout
- **Zero unsafe code**

## Requirements

- Rust 2024 edition (1.85+)
- A running [Docling Serve](https://github.com/docling-project/docling-serve) instance

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
docling_rs = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

### Health check

```rust
use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    let health = client.health().await?;
    println!("Status: {}", health.status); // "ok"
    Ok(())
}
```

### Convert a URL to Markdown (synchronous)

```rust
use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    let result = client
        .convert_source("https://arxiv.org/pdf/2206.01062", None)
        .await?;

    println!("Status: {:?}", result.status);
    if let Some(md) = &result.document.md_content {
        println!("{}", md);
    }
    Ok(())
}
```

### Convert with options

```rust
use docling_rs::{DoclingClient, ConvertDocumentsRequestOptions, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    let opts = ConvertDocumentsRequestOptions {
        to_formats: Some(vec![OutputFormat::Md, OutputFormat::Text]),
        do_ocr: Some(true),
        ..Default::default()
    };

    let result = client
        .convert_source("https://arxiv.org/pdf/2206.01062", Some(opts))
        .await?;

    if let Some(text) = &result.document.text_content {
        println!("{}", text);
    }
    Ok(())
}
```

### Async conversion with polling

For large documents, use async conversion to avoid HTTP timeouts:

```rust
use std::time::Duration;
use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    // Submit, poll, and return result — all in one call
    let result = client
        .wait_for_conversion(
            "https://arxiv.org/pdf/2206.01062",
            None,                        // default options
            Duration::from_secs(300),    // 5 min timeout
            Some(5.0),                   // 5s server-side long-poll
        )
        .await?;

    println!("Status: {:?}", result.status);
    println!("Time: {:.1}s", result.processing_time);
    Ok(())
}
```

### Manual async flow (fine-grained control)

```rust
use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    // 1. Submit
    let task = client
        .convert_source_async("https://arxiv.org/pdf/2206.01062", None)
        .await?;
    println!("Task ID: {}", task.task_id);

    // 2. Poll (with 10s long-poll)
    loop {
        let status = client.poll_task_status(&task.task_id, Some(10.0)).await?;
        println!("Status: {}", status.task_status);

        if status.task_status == "SUCCESS" {
            break;
        }
        if status.task_status == "FAILURE" {
            eprintln!("Task failed!");
            return Ok(());
        }
    }

    // 3. Fetch result
    let result = client.get_task_result(&task.task_id).await?;
    if let Some(md) = &result.document.md_content {
        println!("{}", md);
    }
    Ok(())
}
```

### Convert a local file

```rust
use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    let result = client
        .convert_file(&["./document.pdf"], None, None)
        .await?;

    if let Some(md) = &result.document.md_content {
        println!("{}", md);
    }
    Ok(())
}
```

### Async file conversion with polling

```rust
use std::time::Duration;
use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    let result = client
        .wait_for_file_conversion(
            &["./document.pdf"],
            None,                        // default options
            None,                        // default target (in-body)
            Duration::from_secs(300),    // 5 min timeout
            Some(5.0),                   // 5s server-side long-poll
        )
        .await?;

    println!("Status: {:?}", result.status);
    Ok(())
}
```

### With API key authentication

```rust
use docling_rs::DoclingClient;

let client = DoclingClient::with_api_key(
    "https://docling.example.com",
    "your-api-key-here",
);
```

## API Reference

### `DoclingClient`

| Method | Endpoint | Description |
|---|---|---|
| `health()` | `GET /health` | Health check |
| `version()` | `GET /version` | Server version info |
| `convert_source(url, options)` | `POST /v1/convert/source` | Synchronous URL conversion |
| `convert(request)` | `POST /v1/convert/source` | Full request control |
| `convert_source_async(url, options)` | `POST /v1/convert/source/async` | Submit async URL task |
| `convert_async(request)` | `POST /v1/convert/source/async` | Submit async (full request) |
| `convert_file(paths, options, target)` | `POST /v1/convert/file` | Synchronous file upload |
| `convert_file_async(paths, options, target)` | `POST /v1/convert/file/async` | Submit async file task |
| `poll_task_status(task_id, wait)` | `GET /v1/status/poll/{id}` | Poll task status |
| `get_task_result(task_id)` | `GET /v1/result/{id}` | Fetch completed result |
| `wait_for_conversion(url, opts, timeout, poll)` | (composite) | Submit URL + poll + fetch |
| `wait_for_file_conversion(paths, opts, tgt, timeout, poll)` | (composite) | Submit file + poll + fetch |

### Key Types

**Enums:**
`InputFormat`, `OutputFormat`, `ImageRefMode`, `TableFormerMode`, `PdfBackend`, `ProcessingPipeline`, `OcrEngine`, `ConversionStatus`, `VlmModelType`

**Request:**
`Source` (Http, File), `Target` (InBody, Zip), `ConvertDocumentsRequestOptions`, `ConvertDocumentsRequest`

**Response:**
`ConvertDocumentResponse`, `ExportDocumentResponse`, `TaskStatusResponse`, `HealthCheckResponse`

**Errors:**
`DoclingError` — `Http`, `Api`, `Json`, `Io`, `TaskFailed`, `Timeout`

### Conversion Options

All fields in `ConvertDocumentsRequestOptions` are optional. The server applies defaults for anything omitted. Key options:

| Field | Type | Default | Description |
|---|---|---|---|
| `to_formats` | `Vec<OutputFormat>` | `["md"]` | Output format(s) |
| `do_ocr` | `bool` | `true` | Enable OCR |
| `ocr_engine` | `OcrEngine` | `easyocr` | OCR engine |
| `table_mode` | `TableFormerMode` | `accurate` | Table extraction mode |
| `pdf_backend` | `PdfBackend` | `dlparse_v4` | PDF parser backend |
| `pipeline` | `ProcessingPipeline` | `standard` | Processing pipeline |
| `page_range` | `(i64, i64)` | `(1, MAX)` | Page range to convert |
| `image_export_mode` | `ImageRefMode` | `embedded` | How to handle images |

See `ConvertDocumentsRequestOptions` for the full list (25+ fields).

## Running Examples

Start Docling Serve:

```bash
docling-serve run --port 5001
```

Run examples:

```bash
cargo run --example health
cargo run --example convert_url
cargo run --example convert_url_async
cargo run --example convert_file
cargo run --example convert_file_async
```

## Error Handling

```rust
use docling_rs::{DoclingClient, DoclingError};

async fn example() {
    let client = DoclingClient::new("http://127.0.0.1:5001");

    match client.convert_source("https://example.com/doc.pdf", None).await {
        Ok(result) => println!("Success: {:?}", result.status),
        Err(DoclingError::Http(e)) => eprintln!("Network error: {}", e),
        Err(DoclingError::Api { status_code, body }) => {
            eprintln!("API error {}: {}", status_code, body);
        }
        Err(DoclingError::Json(e)) => eprintln!("Parse error: {}", e),
        Err(DoclingError::Io(e)) => eprintln!("File I/O error: {}", e),
        Err(DoclingError::TaskFailed { task_id, status }) => {
            eprintln!("Task {} failed: {}", task_id, status);
        }
        Err(DoclingError::Timeout { task_id, elapsed_secs }) => {
            eprintln!("Task {} timed out after {:.0}s", task_id, elapsed_secs);
        }
    }
}
```

## Testing

Run the full test suite (no running Docling Serve instance required):

```bash
cargo test
```

The suite includes 79 tests across serialization round-trips, mock HTTP client behavior, auth header handling, file uploads, error mapping, and more.

## License

MIT
