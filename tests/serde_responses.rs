//! Deserialization tests for API response types.
//!
//! These test that our Rust types can correctly parse the JSON that
//! Docling Serve actually sends back.

use serde_json::json;

use docling_rs::models::responses::*;

// ============================================================================
// ConvertDocumentResponse
// ============================================================================

#[test]
fn convert_response_from_realistic_json() {
    let json = json!({
        "document": {
            "filename": "paper.pdf",
            "md_content": "# Title\n\nSome content.",
            "json_content": null,
            "html_content": null,
            "text_content": null,
            "doctags_content": null
        },
        "status": "success",
        "errors": [],
        "processing_time": 2.567,
        "timings": {}
    });

    let resp: ConvertDocumentResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.document.filename, "paper.pdf");
    assert_eq!(
        resp.document.md_content.as_deref(),
        Some("# Title\n\nSome content.")
    );
    assert!(resp.document.json_content.is_none());
    assert!(resp.document.html_content.is_none());
    assert_eq!(resp.processing_time, 2.567);
    assert!(resp.errors.is_empty());
    assert!(resp.timings.is_empty());
}

#[test]
fn convert_response_with_errors_and_timings() {
    let json = json!({
        "document": {
            "filename": "bad.pdf",
            "md_content": null,
            "json_content": null,
            "html_content": null,
            "text_content": null,
            "doctags_content": null
        },
        "status": "partial_success",
        "errors": [
            {
                "component_type": "document_backend",
                "module_name": "pdf_parser",
                "error_message": "Failed to parse page 3"
            }
        ],
        "processing_time": 5.0,
        "timings": {
            "pdf_parse": {
                "scope": "document",
                "count": 1,
                "times": [3.2],
                "start_timestamps": ["2024-01-01T00:00:00"]
            }
        }
    });

    let resp: ConvertDocumentResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.errors.len(), 1);
    assert_eq!(resp.errors[0].module_name, "pdf_parser");
    assert_eq!(resp.errors[0].error_message, "Failed to parse page 3");
    assert_eq!(resp.timings.len(), 1);
    assert!(resp.timings.contains_key("pdf_parse"));

    let timing = &resp.timings["pdf_parse"];
    assert_eq!(timing.count, 1);
    assert_eq!(timing.times, vec![3.2]);
}

#[test]
fn convert_response_missing_optional_fields() {
    // Server might omit optional fields entirely instead of sending null
    let json = json!({
        "document": {
            "filename": "test.pdf"
        },
        "status": "success",
        "processing_time": 1.0
    });

    let resp: ConvertDocumentResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.document.filename, "test.pdf");
    assert!(resp.document.md_content.is_none());
    assert!(resp.document.json_content.is_none());
    // errors and timings should default to empty
    assert!(resp.errors.is_empty());
    assert!(resp.timings.is_empty());
}

// ============================================================================
// TaskStatusResponse
// ============================================================================

#[test]
fn task_status_response_pending() {
    let json = json!({
        "task_id": "abc-123",
        "task_type": "convert",
        "task_status": "PENDING",
        "task_position": 3,
        "task_meta": null
    });

    let resp: TaskStatusResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.task_id, "abc-123");
    assert_eq!(resp.task_status, "PENDING");
    assert_eq!(resp.task_position, Some(3));
    assert!(resp.task_meta.is_none());
}

#[test]
fn task_status_response_success_with_meta() {
    let json = json!({
        "task_id": "xyz-456",
        "task_type": "convert",
        "task_status": "SUCCESS",
        "task_position": null,
        "task_meta": {
            "num_docs": 2,
            "num_processed": 2,
            "num_succeeded": 2,
            "num_failed": 0
        }
    });

    let resp: TaskStatusResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.task_status, "SUCCESS");
    assert!(resp.task_position.is_none());

    let meta = resp.task_meta.unwrap();
    assert_eq!(meta.num_docs, 2);
    assert_eq!(meta.num_processed, 2);
    assert_eq!(meta.num_succeeded, 2);
    assert_eq!(meta.num_failed, 0);
}

#[test]
fn task_status_response_without_optional_fields() {
    // Minimal response — only required fields
    let json = json!({
        "task_id": "min-001",
        "task_type": "convert",
        "task_status": "STARTED"
    });

    let resp: TaskStatusResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.task_id, "min-001");
    assert_eq!(resp.task_status, "STARTED");
    assert!(resp.task_position.is_none());
    assert!(resp.task_meta.is_none());
}

// ============================================================================
// HealthCheckResponse
// ============================================================================

#[test]
fn health_response_normal() {
    let json = json!({"status": "ok"});
    let resp: HealthCheckResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.status, "ok");
}

#[test]
fn health_response_empty_json_uses_default() {
    // The server might return {} — our serde default should fill in "ok"
    let json = json!({});
    let resp: HealthCheckResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.status, "ok");
}

// ============================================================================
// HttpValidationError (422 responses)
// ============================================================================

#[test]
fn validation_error_with_mixed_loc_types() {
    let json = json!({
        "detail": [
            {
                "loc": ["body", "sources", 0],
                "msg": "field required",
                "type": "missing"
            },
            {
                "loc": ["body", "options", "to_formats"],
                "msg": "value is not a valid list",
                "type": "type_error.list"
            }
        ]
    });

    let resp: HttpValidationError = serde_json::from_value(json).unwrap();
    assert_eq!(resp.detail.len(), 2);
    assert_eq!(resp.detail[0].msg, "field required");
    assert_eq!(resp.detail[0].error_type, "missing");
    // loc contains mixed string and integer values
    assert_eq!(resp.detail[0].loc[0], "body");
    assert_eq!(resp.detail[0].loc[2], 0);
}

#[test]
fn validation_error_empty_detail() {
    let json = json!({"detail": []});
    let resp: HttpValidationError = serde_json::from_value(json).unwrap();
    assert!(resp.detail.is_empty());
}

// ============================================================================
// ExportDocumentResponse
// ============================================================================

#[test]
fn export_response_with_multiple_formats() {
    let json = json!({
        "filename": "multi.pdf",
        "md_content": "# Hello",
        "json_content": {"version": "1.0", "body": {}},
        "html_content": "<h1>Hello</h1>",
        "text_content": "Hello",
        "doctags_content": "<doc>Hello</doc>"
    });

    let resp: ExportDocumentResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.filename, "multi.pdf");
    assert_eq!(resp.md_content.as_deref(), Some("# Hello"));
    assert!(resp.json_content.is_some());
    assert_eq!(resp.html_content.as_deref(), Some("<h1>Hello</h1>"));
    assert_eq!(resp.text_content.as_deref(), Some("Hello"));
    assert_eq!(resp.doctags_content.as_deref(), Some("<doc>Hello</doc>"));
}

// ============================================================================
// PresignedUrlConvertDocumentResponse
// ============================================================================

#[test]
fn presigned_url_response() {
    let json = json!({
        "processing_time": 10.5,
        "num_converted": 5,
        "num_succeeded": 4,
        "num_failed": 1
    });

    let resp: PresignedUrlConvertDocumentResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.processing_time, 10.5);
    assert_eq!(resp.num_converted, 5);
    assert_eq!(resp.num_succeeded, 4);
    assert_eq!(resp.num_failed, 1);
}
