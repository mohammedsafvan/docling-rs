use thiserror::Error;

/// Errors that can occur when using the Docling SDK.
#[derive(Error, Debug)]
pub enum DoclingError {
    /// Network-level or HTTP client error (connection refused, DNS failure, etc.).
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// The server returned a non-success HTTP status code.
    #[error("api error (HTTP {status_code}): {body}")]
    Api { status_code: u16, body: String },

    /// Failed to deserialize the response body.
    #[error("json deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// File I/O error (e.g. reading a local file for upload).
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// An async task failed on the server.
    #[error("task {task_id} failed with status: {status}")]
    TaskFailed { task_id: String, status: String },

    /// Timed out waiting for an async task to complete.
    #[error("task {task_id} timed out after {elapsed_secs:.1}s")]
    Timeout { task_id: String, elapsed_secs: f64 },
}
