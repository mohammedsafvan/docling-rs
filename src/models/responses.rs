use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::enums::*;

// ============================================================================
// Response types
// ============================================================================

/// The converted document content in various formats.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExportDocumentResponse {
    /// Original filename.
    pub filename: String,

    /// Markdown content (if requested).
    pub md_content: Option<String>,

    /// Structured JSON content (if requested). Uses generic Value
    /// because DoclingDocument is a very deep schema.
    pub json_content: Option<serde_json::Value>,

    /// HTML content (if requested).
    pub html_content: Option<String>,

    /// Plain text content (if requested).
    pub text_content: Option<String>,

    /// DocTags content (if requested).
    pub doctags_content: Option<String>,
}

/// An error that occurred during conversion.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorItem {
    pub component_type: DoclingComponentType,
    pub module_name: String,
    pub error_message: String,
}

/// Profiling information for a conversion step.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfilingItem {
    pub scope: ProfilingScope,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub times: Vec<f64>,
    #[serde(default)]
    pub start_timestamps: Vec<String>,
}

/// Response from `POST /v1/convert/source` (synchronous conversion).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConvertDocumentResponse {
    /// The converted document.
    pub document: ExportDocumentResponse,

    /// Conversion status.
    pub status: ConversionStatus,

    /// Errors encountered during conversion.
    #[serde(default)]
    pub errors: Vec<ErrorItem>,

    /// Total processing time in seconds.
    pub processing_time: f64,

    /// Detailed profiling timings.
    #[serde(default)]
    pub timings: HashMap<String, ProfilingItem>,
}

/// Response when target is S3/presigned URL (not in-body).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PresignedUrlConvertDocumentResponse {
    pub processing_time: f64,
    pub num_converted: i64,
    pub num_succeeded: i64,
    pub num_failed: i64,
}

// ============================================================================
// Async task types
// ============================================================================

/// Processing metadata for an async task.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskProcessingMeta {
    pub num_docs: i64,
    #[serde(default)]
    pub num_processed: i64,
    #[serde(default)]
    pub num_succeeded: i64,
    #[serde(default)]
    pub num_failed: i64,
}

/// Response from async task submission and status polling.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskStatusResponse {
    /// Unique identifier for the task.
    pub task_id: String,

    /// Type of task (convert or chunk).
    pub task_type: TaskType,

    /// Current status string (e.g. "PENDING", "SUCCESS", "FAILURE").
    pub task_status: String,

    /// Position in queue (if waiting).
    pub task_position: Option<i64>,

    /// Processing progress metadata.
    pub task_meta: Option<TaskProcessingMeta>,
}

// ============================================================================
// Health / version
// ============================================================================

/// Response from `GET /health`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthCheckResponse {
    #[serde(default = "default_health_status")]
    pub status: String,
}

fn default_health_status() -> String {
    "ok".to_string()
}

// ============================================================================
// Validation error types (HTTP 422 responses)
// ============================================================================

/// A single validation error detail from the server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidationErrorDetail {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: String,
}

/// HTTP validation error response (422).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpValidationError {
    #[serde(default)]
    pub detail: Vec<ValidationErrorDetail>,
}
