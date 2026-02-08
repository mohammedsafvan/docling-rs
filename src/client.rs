use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

use reqwest::multipart::{Form, Part};

use crate::error::DoclingError;
use crate::models::*;

/// Async HTTP client for Docling Serve.
pub struct DoclingClient {
    base_url: String,
    api_key: Option<String>,
    http: reqwest::Client,
}

impl DoclingClient {
    /// Create a new client pointing at the given Docling Serve base URL.
    ///
    /// ```rust
    /// use docling_rs::DoclingClient;
    /// let client = DoclingClient::new("http://127.0.0.1:5001");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        let base_url = base_url.into();
        let base_url = base_url.trim_end_matches('/').to_string();
        Self {
            base_url,
            api_key: None,
            http: reqwest::Client::new(),
        }
    }

    /// Create a new client with API key authentication.
    ///
    /// The key is sent as `Authorization: Bearer <key>` on every request
    /// to secured endpoints.
    pub fn with_api_key(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        let base_url = base_url.into();
        let base_url = base_url.trim_end_matches('/').to_string();
        Self {
            base_url,
            api_key: Some(api_key.into()),
            http: reqwest::Client::new(),
        }
    }

    // ========================================================================
    // Internal helpers
    // ========================================================================

    /// Build a full URL from a path.
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Apply authorization header if an API key is configured.
    fn auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.api_key {
            Some(key) => req.bearer_auth(key),
            None => req,
        }
    }

    /// Send a request and handle non-success status codes by reading the
    /// body and returning a structured `DoclingError::Api`.
    async fn handle_response(
        &self,
        response: reqwest::Response,
    ) -> Result<reqwest::Response, DoclingError> {
        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else {
            let status_code = status.as_u16();
            let body = response.text().await.unwrap_or_default();
            Err(DoclingError::Api { status_code, body })
        }
    }

    /// Poll an async task until it completes, fails, or times out.
    ///
    /// This is the shared implementation used by both [`wait_for_conversion`]
    /// and [`wait_for_file_conversion`] to avoid duplicated polling logic.
    async fn poll_until_complete(
        &self,
        task_id: &str,
        timeout: Duration,
        poll_interval_secs: Option<f64>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        let poll_wait = poll_interval_secs.unwrap_or(5.0);
        let start = Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(DoclingError::Timeout {
                    task_id: task_id.to_string(),
                    elapsed_secs: start.elapsed().as_secs_f64(),
                });
            }

            let status = self.poll_task_status(task_id, Some(poll_wait)).await?;

            match status.task_status.as_str() {
                "SUCCESS" => {
                    return self.get_task_result(task_id).await;
                }
                "FAILURE" => {
                    return Err(DoclingError::TaskFailed {
                        task_id: task_id.to_string(),
                        status: "FAILURE".to_string(),
                    });
                }
                // PENDING, STARTED, or any other status — keep polling
                _ => continue,
            }
        }
    }

    // ========================================================================
    // Health & Version
    // ========================================================================

    /// Check if the Docling Serve instance is healthy.
    ///
    /// `GET /health`
    pub async fn health(&self) -> Result<HealthCheckResponse, DoclingError> {
        let resp = self.http.get(self.url("/health")).send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<HealthCheckResponse>().await?;
        Ok(body)
    }

    /// Get version information from the Docling Serve instance.
    ///
    /// `GET /version`
    pub async fn version(&self) -> Result<HashMap<String, serde_json::Value>, DoclingError> {
        let resp = self.http.get(self.url("/version")).send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<HashMap<String, serde_json::Value>>().await?;
        Ok(body)
    }

    // ========================================================================
    // Synchronous URL conversion
    // ========================================================================

    /// Convert a document from a URL (synchronous).
    ///
    /// This is the simplest way to convert a document. The call blocks (async)
    /// until the conversion is complete and returns the result directly.
    ///
    /// `POST /v1/convert/source`
    ///
    /// # Arguments
    /// * `url` — The HTTP URL of the document to convert.
    /// * `options` — Optional conversion options. Pass `None` for server defaults.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = docling_rs::client::DoclingClient::new("http://127.0.0.1:5001");
    /// let result = client
    ///     .convert_source("https://arxiv.org/pdf/2206.01062", None)
    ///     .await?;
    /// if let Some(md) = &result.document.md_content {
    ///     println!("{}", md);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn convert_source(
        &self,
        url: &str,
        options: Option<ConvertDocumentsRequestOptions>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        let request_body = ConvertDocumentsRequest {
            sources: vec![Source::Http {
                url: url.to_string(),
                headers: None,
            }],
            options,
            target: None, // defaults to InBody
        };

        let req = self.auth(
            self.http
                .post(self.url("/v1/convert/source"))
                .json(&request_body),
        );

        let resp = req.send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<ConvertDocumentResponse>().await?;
        Ok(body)
    }

    /// Convert documents from multiple sources (synchronous).
    ///
    /// `POST /v1/convert/source`
    ///
    /// Use this when you need full control over sources, options, and target.
    pub async fn convert(
        &self,
        request: &ConvertDocumentsRequest,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        let req = self.auth(
            self.http
                .post(self.url("/v1/convert/source"))
                .json(request),
        );

        let resp = req.send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<ConvertDocumentResponse>().await?;
        Ok(body)
    }

    // ========================================================================
    // Async URL conversion
    // ========================================================================

    /// Submit a document for asynchronous conversion.
    ///
    /// Returns a `TaskStatusResponse` containing the `task_id` which can be
    /// used to poll for status and retrieve results.
    ///
    /// `POST /v1/convert/source/async`
    pub async fn convert_source_async(
        &self,
        url: &str,
        options: Option<ConvertDocumentsRequestOptions>,
    ) -> Result<TaskStatusResponse, DoclingError> {
        let request_body = ConvertDocumentsRequest {
            sources: vec![Source::Http {
                url: url.to_string(),
                headers: None,
            }],
            options,
            target: None,
        };

        let req = self.auth(
            self.http
                .post(self.url("/v1/convert/source/async"))
                .json(&request_body),
        );

        let resp = req.send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<TaskStatusResponse>().await?;
        Ok(body)
    }

    /// Submit a full request for asynchronous conversion.
    ///
    /// `POST /v1/convert/source/async`
    pub async fn convert_async(
        &self,
        request: &ConvertDocumentsRequest,
    ) -> Result<TaskStatusResponse, DoclingError> {
        let req = self.auth(
            self.http
                .post(self.url("/v1/convert/source/async"))
                .json(request),
        );

        let resp = req.send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<TaskStatusResponse>().await?;
        Ok(body)
    }

    // ========================================================================
    // Task polling & result retrieval
    // ========================================================================

    /// Poll the status of an async task.
    ///
    /// `GET /v1/status/poll/{task_id}?wait=N`
    ///
    /// # Arguments
    /// * `task_id` — The task ID from `convert_source_async`.
    /// * `wait_secs` — Optional long-poll duration. The server will hold the
    ///   connection open for up to this many seconds waiting for completion.
    ///   Pass `None` or `Some(0.0)` for an immediate status check.
    pub async fn poll_task_status(
        &self,
        task_id: &str,
        wait_secs: Option<f64>,
    ) -> Result<TaskStatusResponse, DoclingError> {
        let mut url = self.url(&format!("/v1/status/poll/{}", task_id));
        if let Some(w) = wait_secs {
            url = format!("{}?wait={}", url, w);
        }

        let req = self.auth(self.http.get(&url));
        let resp = req.send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<TaskStatusResponse>().await?;
        Ok(body)
    }

    /// Retrieve the result of a completed async task.
    ///
    /// `GET /v1/result/{task_id}`
    ///
    /// This should only be called after `poll_task_status` indicates the
    /// task has completed (status = "SUCCESS").
    pub async fn get_task_result(
        &self,
        task_id: &str,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        let req = self.auth(
            self.http
                .get(self.url(&format!("/v1/result/{}", task_id))),
        );

        let resp = req.send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<ConvertDocumentResponse>().await?;
        Ok(body)
    }

    // ========================================================================
    // Convenience: submit URL + wait
    // ========================================================================

    /// Submit an async conversion and wait for it to complete.
    ///
    /// This is a convenience method that combines `convert_source_async`,
    /// polling via `poll_task_status`, and `get_task_result` into a single
    /// call. The method polls using server-side long-polling for efficiency.
    ///
    /// # Arguments
    /// * `url` — The HTTP URL of the document to convert.
    /// * `options` — Optional conversion options.
    /// * `timeout` — Maximum time to wait for completion.
    /// * `poll_interval_secs` — Server-side long-poll wait time per request.
    ///   Defaults to 5 seconds if `None`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use std::time::Duration;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = docling_rs::client::DoclingClient::new("http://127.0.0.1:5001");
    /// let result = client
    ///     .wait_for_conversion(
    ///         "https://arxiv.org/pdf/2206.01062",
    ///         None,
    ///         Duration::from_secs(300),
    ///         None,
    ///     )
    ///     .await?;
    /// println!("Status: {:?}", result.status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_conversion(
        &self,
        url: &str,
        options: Option<ConvertDocumentsRequestOptions>,
        timeout: Duration,
        poll_interval_secs: Option<f64>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        let task = self.convert_source_async(url, options).await?;
        self.poll_until_complete(&task.task_id, timeout, poll_interval_secs)
            .await
    }

    // ========================================================================
    // Multipart file upload
    // ========================================================================

    /// Build a `multipart/form-data` form from file paths and conversion options.
    ///
    /// Each file is read from disk and attached as a binary part named `files`.
    /// Each conversion option (if set) is added as a text form field using the
    /// same field names as the OpenAPI spec. Array fields (e.g. `from_formats`,
    /// `to_formats`, `ocr_lang`) are sent as repeated form fields, which is how
    /// FastAPI parses multipart list parameters.
    async fn build_file_multipart(
        &self,
        file_paths: &[impl AsRef<Path>],
        options: Option<&ConvertDocumentsRequestOptions>,
        target_type: Option<&TargetName>,
    ) -> Result<Form, DoclingError> {
        let mut form = Form::new();

        // Attach each file as a binary part
        for path in file_paths {
            let path = path.as_ref();
            let bytes = tokio::fs::read(path).await.map_err(DoclingError::Io)?;
            let filename = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "file".to_string());

            // Guess MIME type from extension
            let mime = match path.extension().and_then(|e| e.to_str()) {
                Some("pdf") => "application/pdf",
                Some("docx") => {
                    "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                }
                Some("pptx") => {
                    "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                }
                Some("xlsx") => {
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                }
                Some("html") | Some("htm") => "text/html",
                Some("md") => "text/markdown",
                Some("csv") => "text/csv",
                Some("json") => "application/json",
                Some("xml") => "application/xml",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("tiff") | Some("tif") => "image/tiff",
                Some("bmp") => "image/bmp",
                Some("webp") => "image/webp",
                Some("mp3") => "audio/mpeg",
                Some("wav") => "audio/wav",
                Some("vtt") => "text/vtt",
                _ => "application/octet-stream",
            };

            let part = Part::bytes(bytes)
                .file_name(filename)
                .mime_str(mime)
                .unwrap();
            form = form.part("files", part);
        }

        // Add target_type
        if let Some(tt) = target_type {
            form = form.text("target_type", tt.to_string());
        }

        // Add options as flat form fields
        if let Some(opts) = options {
            // Array fields — sent as repeated form fields for FastAPI
            if let Some(ref fmts) = opts.from_formats {
                for fmt in fmts {
                    form = form.text("from_formats", fmt.to_string());
                }
            }
            if let Some(ref fmts) = opts.to_formats {
                for fmt in fmts {
                    form = form.text("to_formats", fmt.to_string());
                }
            }
            if let Some(ref langs) = opts.ocr_lang {
                for lang in langs {
                    form = form.text("ocr_lang", lang.clone());
                }
            }
            if let Some(ref range) = opts.page_range {
                form = form.text("page_range", range.0.to_string());
                form = form.text("page_range", range.1.to_string());
            }

            // Enum fields
            if let Some(ref v) = opts.image_export_mode {
                form = form.text("image_export_mode", v.to_string());
            }
            if let Some(ref v) = opts.ocr_engine {
                form = form.text("ocr_engine", v.to_string());
            }
            if let Some(ref v) = opts.pdf_backend {
                form = form.text("pdf_backend", v.to_string());
            }
            if let Some(ref v) = opts.table_mode {
                form = form.text("table_mode", v.to_string());
            }
            if let Some(ref v) = opts.pipeline {
                form = form.text("pipeline", v.to_string());
            }
            if let Some(ref v) = opts.vlm_pipeline_model {
                form = form.text("vlm_pipeline_model", v.to_string());
            }

            // Boolean fields
            if let Some(v) = opts.do_ocr {
                form = form.text("do_ocr", v.to_string());
            }
            if let Some(v) = opts.force_ocr {
                form = form.text("force_ocr", v.to_string());
            }
            if let Some(v) = opts.table_cell_matching {
                form = form.text("table_cell_matching", v.to_string());
            }
            if let Some(v) = opts.abort_on_error {
                form = form.text("abort_on_error", v.to_string());
            }
            if let Some(v) = opts.do_table_structure {
                form = form.text("do_table_structure", v.to_string());
            }
            if let Some(v) = opts.include_images {
                form = form.text("include_images", v.to_string());
            }
            if let Some(v) = opts.do_code_enrichment {
                form = form.text("do_code_enrichment", v.to_string());
            }
            if let Some(v) = opts.do_formula_enrichment {
                form = form.text("do_formula_enrichment", v.to_string());
            }
            if let Some(v) = opts.do_picture_classification {
                form = form.text("do_picture_classification", v.to_string());
            }
            if let Some(v) = opts.do_chart_extraction {
                form = form.text("do_chart_extraction", v.to_string());
            }
            if let Some(v) = opts.do_picture_description {
                form = form.text("do_picture_description", v.to_string());
            }

            // Numeric fields
            if let Some(v) = opts.document_timeout {
                form = form.text("document_timeout", v.to_string());
            }
            if let Some(v) = opts.images_scale {
                form = form.text("images_scale", v.to_string());
            }
            if let Some(v) = opts.picture_description_area_threshold {
                form = form.text("picture_description_area_threshold", v.to_string());
            }

            // String fields
            if let Some(ref v) = opts.md_page_break_placeholder {
                form = form.text("md_page_break_placeholder", v.clone());
            }

            // JSON-encoded object fields (sent as JSON strings in multipart)
            if let Some(ref v) = opts.picture_description_local {
                form = form.text("picture_description_local", v.to_string());
            }
            if let Some(ref v) = opts.picture_description_api {
                form = form.text("picture_description_api", v.to_string());
            }
            if let Some(ref v) = opts.vlm_pipeline_model_local {
                form = form.text("vlm_pipeline_model_local", v.to_string());
            }
            if let Some(ref v) = opts.vlm_pipeline_model_api {
                form = form.text("vlm_pipeline_model_api", v.to_string());
            }
        }

        Ok(form)
    }

    /// Convert one or more local files (synchronous).
    ///
    /// Reads each file from disk and uploads via `multipart/form-data`.
    /// The call blocks (async) until conversion is complete.
    ///
    /// `POST /v1/convert/file`
    ///
    /// # Arguments
    /// * `file_paths` — One or more local file paths to convert.
    /// * `options` — Optional conversion options. Pass `None` for server defaults.
    /// * `target_type` — Optional target type. Pass `None` for default (in-body).
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = docling_rs::client::DoclingClient::new("http://127.0.0.1:5001");
    /// let result = client
    ///     .convert_file(&["./document.pdf"], None, None)
    ///     .await?;
    /// if let Some(md) = &result.document.md_content {
    ///     println!("{}", md);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn convert_file(
        &self,
        file_paths: &[impl AsRef<Path>],
        options: Option<&ConvertDocumentsRequestOptions>,
        target_type: Option<&TargetName>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        let form = self
            .build_file_multipart(file_paths, options, target_type)
            .await?;

        let req = self.auth(
            self.http
                .post(self.url("/v1/convert/file"))
                .multipart(form),
        );

        let resp = req.send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<ConvertDocumentResponse>().await?;
        Ok(body)
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
    pub async fn convert_file_async(
        &self,
        file_paths: &[impl AsRef<Path>],
        options: Option<&ConvertDocumentsRequestOptions>,
        target_type: Option<&TargetName>,
    ) -> Result<TaskStatusResponse, DoclingError> {
        let form = self
            .build_file_multipart(file_paths, options, target_type)
            .await?;

        let req = self.auth(
            self.http
                .post(self.url("/v1/convert/file/async"))
                .multipart(form),
        );

        let resp = req.send().await?;
        let resp = self.handle_response(resp).await?;
        let body = resp.json::<TaskStatusResponse>().await?;
        Ok(body)
    }

    // ========================================================================
    // Convenience: submit file + wait
    // ========================================================================

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
    ///
    /// # Example
    /// ```rust,no_run
    /// # use std::time::Duration;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = docling_rs::client::DoclingClient::new("http://127.0.0.1:5001");
    /// let result = client
    ///     .wait_for_file_conversion(
    ///         &["./document.pdf"],
    ///         None,
    ///         None,
    ///         Duration::from_secs(300),
    ///         None,
    ///     )
    ///     .await?;
    /// println!("Status: {:?}", result.status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_file_conversion(
        &self,
        file_paths: &[impl AsRef<Path>],
        options: Option<&ConvertDocumentsRequestOptions>,
        target_type: Option<&TargetName>,
        timeout: Duration,
        poll_interval_secs: Option<f64>,
    ) -> Result<ConvertDocumentResponse, DoclingError> {
        let task = self
            .convert_file_async(file_paths, options, target_type)
            .await?;
        self.poll_until_complete(&task.task_id, timeout, poll_interval_secs)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_no_trailing_slash() {
        let client = DoclingClient::new("http://localhost:5001");
        assert_eq!(client.url("/health"), "http://localhost:5001/health");
    }

    #[test]
    fn url_trailing_slash_stripped() {
        let client = DoclingClient::new("http://localhost:5001/");
        assert_eq!(client.url("/health"), "http://localhost:5001/health");
    }

    #[test]
    fn url_multiple_trailing_slashes_stripped() {
        let client = DoclingClient::new("http://localhost:5001///");
        assert_eq!(client.url("/health"), "http://localhost:5001/health");
    }

    #[test]
    fn url_deep_path() {
        let client = DoclingClient::new("http://localhost:5001");
        assert_eq!(
            client.url("/v1/convert/source"),
            "http://localhost:5001/v1/convert/source"
        );
    }

    #[test]
    fn with_api_key_also_strips_trailing_slash() {
        let client = DoclingClient::with_api_key("http://localhost:5001/", "key");
        assert_eq!(client.url("/health"), "http://localhost:5001/health");
        assert_eq!(client.api_key.as_deref(), Some("key"));
    }
}
