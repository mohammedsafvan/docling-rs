# docling_rs

Rust SDK for [Docling Serve](https://github.com/docling-project/docling-serve). Convert PDFs, DOCX, PPTX, images, and more into Markdown, JSON, HTML, or plain text.

[![docs.rs](https://img.shields.io/badge/docs.rs-docling__rs-blue)](https://docs.rs/docling_rs)

## Features

- **Async-first HTTP client** — Built on `reqwest` with full async/await support
- **Synchronous API** — Blocking wrapper available via feature flag for simple scripts
- **URL & file conversion** — Convert from HTTP URLs or upload local files via multipart
- **Sync & async job handling** — Block until done, or submit and poll for large documents
- **Fully typed** — All enums and models matching OpenAPI 3.1 spec (v1.12.0)
- **API key authentication** — Bearer token support for secured endpoints
- **Structured errors** — Typed errors for network, API, JSON, I/O, task failures, timeouts
- **Zero unsafe code**

## Installation

```toml
[dependencies]
docling_rs = "0.1"
```

All dependencies (including `tokio` for the internal runtime) are included automatically.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `blocking` | ✅ | Enables synchronous API in `docling_rs::blocking`. |

### Using without blocking API

For smaller binary size when you only need async:

```toml
[dependencies]
docling_rs = { version = "0.1", default-features = false }
```

## Quick Start

### Blocking API (Simplest)

No async boilerplate needed. Works in any Rust program:

```rust
use docling_rs::blocking::DoclingClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");
    
    // Convert a URL
    let result = client
        .convert_source("https://arxiv.org/pdf/2206.01062", None)?;
    
    if let Some(md) = &result.document.md_content {
        println!("{}", md);
    }
    Ok(())
}
```

### Async API

For integration with async Rust code. Requires `tokio` for the async runtime:

```toml
[dependencies]
docling_rs = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust
use docling_rs::DoclingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DoclingClient::new("http://127.0.0.1:5001");
    
    // Convert a URL
    let result = client
        .convert_source("https://arxiv.org/pdf/2206.01062", None)
        .await?;
    
    if let Some(md) = &result.document.md_content {
        println!("{}", md);
    }
    Ok(())
}
```

## Examples

```bash
# Run Docling Serve
docling-serve run --port 5001

# Run examples
cargo run --example health
cargo run --example convert_url
cargo run --example convert_url_async
cargo run --example convert_file
cargo run --example convert_file_async
cargo run --example convert_url_blocking  # No tokio needed
```

## API Overview

### Blocking API

```rust
use docling_rs::blocking::DoclingClient;

let client = DoclingClient::new("http://127.0.0.1:5001");

// Health & version
client.health()?;
client.version()?;

// Basic conversion (options is owned: Option<ConvertDocumentsRequestOptions>)
let result = client.convert_source(url, options)?;

// File upload (options is borrowed: Option<&ConvertDocumentsRequestOptions>)
let result = client.convert_file(paths, options.as_ref(), target)?;

// Async with polling (blocking wrapper)
let task = client.convert_source_async(url, options)?;
let status = client.poll_task_status(&task.task_id, None)?;
let result = client.get_task_result(&task.task_id)?;

// Convenience methods
let result = client.wait_for_conversion(url, options, timeout, poll_interval)?;
let result = client.wait_for_file_conversion(paths, options.as_ref(), target, timeout, poll_interval)?;
```

### Async API

```rust
use docling_rs::DoclingClient;

let client = DoclingClient::new("http://127.0.0.1:5001");

// Health & version
client.health().await?;
client.version().await?;

// Basic conversion (options is owned: Option<ConvertDocumentsRequestOptions>)
let result = client.convert_source(url, options).await?;

// File upload (options is borrowed: Option<&ConvertDocumentsRequestOptions>)
let result = client.convert_file(paths, options.as_ref(), target).await?;

// Async with polling
let task = client.convert_source_async(url, options).await?;
let status = client.poll_task_status(&task.task_id, None).await?;
let result = client.get_task_result(&task.task_id).await?;

// Convenience methods
let result = client.wait_for_conversion(url, options, timeout, poll_interval).await?;
let result = client.wait_for_file_conversion(paths, options.as_ref(), target, timeout, poll_interval).await?;
```

## Error Handling

Both APIs return the same `DoclingError` variants:

```rust
use docling_rs::blocking::DoclingClient;
use docling_rs::DoclingError;

let client = DoclingClient::new("http://127.0.0.1:5001");

match client.convert_source("https://example.com/doc.pdf", None) {
    Ok(result) => println!("Success: {}", result.status),
    Err(DoclingError::Http(e)) => eprintln!("Network error: {}", e),
    Err(DoclingError::Api { status_code, body }) => {
        eprintln!("API error {}: {}", status_code, body);
    }
    Err(DoclingError::TaskFailed { task_id, status }) => {
        eprintln!("Task {} failed: {}", task_id, status);
    }
    Err(DoclingError::Timeout { task_id, elapsed_secs }) => {
        eprintln!("Task {} timed out after {:.0}s", task_id, elapsed_secs);
    }
    _ => eprintln!("Other error"),
}
```

## License

MIT
