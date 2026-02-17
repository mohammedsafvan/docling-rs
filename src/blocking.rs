//! Synchronous/blocking API for docling_rs.
//!
//! This module provides blocking versions of all async operations,
//! using an internal Tokio runtime. Useful for simple scripts or when
//! you don't want to deal with async/await.
//!
//! # Example
//!
//! ```rust,no_run
//! use docling_rs::blocking::DoclingClient;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = DoclingClient::new("http://127.0.0.1:5001");
//!     let result = client.convert_source("https://example.com/doc.pdf", None)?;
//!     println!("Converted: {:?}", result.status);
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use crate::error::DoclingError;
use crate::models::enums::TargetName;
use crate::models::requests::{ConvertDocumentsRequest, ConvertDocumentsRequestOptions};
use crate::models::responses::{ConvertDocumentResponse, HealthCheckResponse, TaskStatusResponse};

/// Synchronous HTTP client for Docling Serve.
///
/// This is a blocking wrapper around the async [`crate::client::DoclingClient`].
/// It uses an internal Tokio runtime to execute async operations synchronously.
pub struct DoclingClient {
    runtime: tokio::runtime::Runtime,
    inner: crate::client::DoclingClient,
}

impl DoclingClient {
    /// Create a new blocking client pointing at the given Docling Serve base URL.
    ///
    /// # Example
    /// ```rust,no_run
    /// use docling_rs::blocking::DoclingClient;
    /// let client = DoclingClient::new("http://127.0.0.1:5001");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        let inner = crate::client::DoclingClient::new(base_url);
        Self { runtime, inner }
    }

    /// Create a new blocking client with API key authentication.
    ///
    /// The key is sent as `Authorization: Bearer <key>` on every request
    /// to secured endpoints.
    pub fn with_api_key(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        let inner = crate::client::DoclingClient::with_api_key(base_url, api_key);
        Self { runtime, inner }
    }

    /// Check if the Docling Serve instance is healthy.
    ///
    /// `GET /health`
    pub fn health(&self) -> Result<HealthCheckResponse, DoclingError> {
        self.runtime.block_on(self.inner.health())
    }

    /// Get version information from the Docling Serve instance.
    ///
    /// `GET /version`
    pub fn version(&self) -> Result<HashMap<String, serde_json::Value>, DoclingError> {
        self.runtime.block_on(self.inner.version())
    }

    /// Convert a document from a URL (synchronous).
    ///
    /// `POST /v1/convert/source`
    ///
    /// # Arguments
    /// * `url` — The HTTP URL of the document to convert.
    /// * `options` — Optional conversion options. Pass `None` for server defaults.
    pub fn convert_source(
        &self,
        url: &str,
        options: Option<ConvertDocumentsRequestOptions>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        self.runtime
            .block_on(self.inner.convert_source(url, options))
    }

    /// Convert documents from multiple sources (synchronous).
    ///
    /// `POST /v1/convert/source`
    ///
    /// Use this when you need full control over sources, options, and target.
    pub fn convert(
        &self,
        request: &ConvertDocumentsRequest,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        self.runtime.block_on(self.inner.convert(request))
    }

    /// Submit a document for asynchronous conversion.
    ///
    /// Returns a `TaskStatusResponse` containing the `task_id` which can be
    /// used to poll for status and retrieve results.
    ///
    /// `POST /v1/convert/source/async`
    pub fn convert_source_async(
        &self,
        url: &str,
        options: Option<ConvertDocumentsRequestOptions>,
    ) -> Result<TaskStatusResponse, DoclingError> {
        self.runtime
            .block_on(self.inner.convert_source_async(url, options))
    }

    /// Submit a full request for asynchronous conversion.
    ///
    /// `POST /v1/convert/source/async`
    pub fn convert_async(
        &self,
        request: &ConvertDocumentsRequest,
    ) -> Result<TaskStatusResponse, DoclingError> {
        self.runtime.block_on(self.inner.convert_async(request))
    }

    /// Poll the status of an async task.
    ///
    /// `GET /v1/status/poll/{task_id}?wait=N`
    ///
    /// # Arguments
    /// * `task_id` — The task ID from `convert_source_async`.
    /// * `wait_secs` — Optional long-poll duration. The server will hold the
    ///   connection open for up to this many seconds waiting for completion.
    ///   Pass `None` or `Some(0.0)` for an immediate status check.
    pub fn poll_task_status(
        &self,
        task_id: &str,
        wait_secs: Option<f64>,
    ) -> Result<TaskStatusResponse, DoclingError> {
        self.runtime
            .block_on(self.inner.poll_task_status(task_id, wait_secs))
    }

    /// Retrieve the result of a completed async task.
    ///
    /// `GET /v1/result/{task_id}`
    ///
    /// This should only be called after `poll_task_status` indicates the
    /// task has completed (status = "SUCCESS").
    pub fn get_task_result(&self, task_id: &str) -> Result<ConvertDocumentResponse, DoclingError> {
        self.runtime.block_on(self.inner.get_task_result(task_id))
    }

    /// Submit an async conversion and wait for it to complete.
    ///
    /// This is a convenience method that combines `convert_source_async`,
    /// polling via `poll_task_status`, and `get_task_result` into a single call.
    ///
    /// # Arguments
    /// * `url` — The HTTP URL of the document to convert.
    /// * `options` — Optional conversion options.
    /// * `timeout` — Maximum time to wait for completion.
    /// * `poll_interval_secs` — Server-side long-poll wait time per request.
    ///   Defaults to 5 seconds if `None`.
    pub fn wait_for_conversion(
        &self,
        url: &str,
        options: Option<ConvertDocumentsRequestOptions>,
        timeout: Duration,
        poll_interval_secs: Option<f64>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        self.runtime.block_on(self.inner.wait_for_conversion(
            url,
            options,
            timeout,
            poll_interval_secs,
        ))
    }

    /// Convert one or more local files (synchronous).
    ///
    /// Reads each file from disk and uploads via `multipart/form-data`.
    /// The call blocks until conversion is complete.
    ///
    /// `POST /v1/convert/file`
    ///
    /// # Arguments
    /// * `file_paths` — One or more local file paths to convert.
    /// * `options` — Optional conversion options. Pass `None` for server defaults.
    /// * `target_type` — Optional target type. Pass `None` for default (in-body).
    pub fn convert_file(
        &self,
        file_paths: &[impl AsRef<Path>],
        options: Option<&ConvertDocumentsRequestOptions>,
        target_type: Option<&TargetName>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        self.runtime
            .block_on(self.inner.convert_file(file_paths, options, target_type))
    }

    /// Submit one or more local files for asynchronous conversion.
    ///
    /// Returns a `TaskStatusResponse` containing the `task_id` which can be
    /// used to poll for status and retrieve results.
    ///
    /// `POST /v1/convert/file/async`
    ///
    /// # Arguments
    /// * `file_paths` — One or more local file paths to convert.
    /// * `options` — Optional conversion options. Pass `None` for server defaults.
    /// * `target_type` — Optional target type. Pass `None` for default (in-body).
    pub fn convert_file_async(
        &self,
        file_paths: &[impl AsRef<Path>],
        options: Option<&ConvertDocumentsRequestOptions>,
        target_type: Option<&TargetName>,
    ) -> Result<TaskStatusResponse, DoclingError> {
        self.runtime.block_on(
            self.inner
                .convert_file_async(file_paths, options, target_type),
        )
    }

    /// Submit local files for async conversion and wait for completion.
    ///
    /// Convenience method that combines `convert_file_async`, polling via
    /// `poll_task_status`, and `get_task_result` into a single call.
    ///
    /// # Arguments
    /// * `file_paths` — One or more local file paths to convert.
    /// * `options` — Optional conversion options.
    /// * `target_type` — Optional target type. Pass `None` for default (in-body).
    /// * `timeout` — Maximum time to wait for completion.
    /// * `poll_interval_secs` — Server-side long-poll wait time per request.
    ///   Defaults to 5 seconds if `None`.
    pub fn wait_for_file_conversion(
        &self,
        file_paths: &[impl AsRef<Path>],
        options: Option<&ConvertDocumentsRequestOptions>,
        target_type: Option<&TargetName>,
        timeout: Duration,
        poll_interval_secs: Option<f64>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        self.runtime.block_on(self.inner.wait_for_file_conversion(
            file_paths,
            options,
            target_type,
            timeout,
            poll_interval_secs,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocking_client_new() {
        let _client = DoclingClient::new("http://localhost:5001");
        // Just verify it creates without panicking
        assert!(true);
    }

    #[test]
    fn blocking_client_with_api_key() {
        let _client = DoclingClient::with_api_key("http://localhost:5001", "test-key");
        // Just verify it creates without panicking
        assert!(true);
    }
}
