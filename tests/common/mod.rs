//! Shared test helpers and JSON fixtures for integration tests.
//!
//! Not every integration test binary uses every helper, so we suppress
//! `dead_code` warnings for the entire module.
#![allow(dead_code)]

use serde_json::{json, Value};

use docling_rs::DoclingClient;

/// Create a `DoclingClient` pointing at the given mock server URL.
pub fn test_client(server_url: &str) -> DoclingClient {
    DoclingClient::new(server_url)
}

/// Create a `DoclingClient` with API key pointing at the given mock server URL.
pub fn test_client_with_key(server_url: &str, api_key: &str) -> DoclingClient {
    DoclingClient::with_api_key(server_url, api_key)
}

/// Build a realistic `ConvertDocumentResponse` JSON matching the API schema.
pub fn convert_response_json() -> Value {
    json!({
        "document": {
            "filename": "test.pdf",
            "md_content": "# Hello World\n\nThis is a test document.",
            "json_content": null,
            "html_content": null,
            "text_content": null,
            "doctags_content": null
        },
        "status": "success",
        "errors": [],
        "processing_time": 1.234,
        "timings": {}
    })
}

/// Build a `TaskStatusResponse` JSON with a configurable status.
pub fn task_status_json(task_id: &str, status: &str) -> Value {
    json!({
        "task_id": task_id,
        "task_type": "convert",
        "task_status": status,
        "task_position": null,
        "task_meta": {
            "num_docs": 1,
            "num_processed": if status == "SUCCESS" { 1 } else { 0 },
            "num_succeeded": if status == "SUCCESS" { 1 } else { 0 },
            "num_failed": if status == "FAILURE" { 1 } else { 0 }
        }
    })
}

/// Build a `HealthCheckResponse` JSON.
pub fn health_response_json() -> Value {
    json!({
        "status": "ok"
    })
}

/// Build a `version` response JSON (free-form HashMap).
pub fn version_response_json() -> Value {
    json!({
        "version": "1.12.0",
        "docling": "2.31.0"
    })
}
