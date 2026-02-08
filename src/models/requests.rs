use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::enums::*;

// ============================================================================
// Source types (discriminated union on "kind")
// ============================================================================

/// A document source — discriminated union on the `kind` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Source {
    /// Fetch document from an HTTP URL.
    #[serde(rename = "http")]
    Http {
        /// The URL to fetch the document from.
        url: String,
        /// Additional headers for the HTTP request (e.g. authorization).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
    },

    /// Inline file as base64-encoded content.
    #[serde(rename = "file")]
    File {
        /// Base64-encoded file content.
        base64_string: String,
        /// Original filename (e.g. "document.pdf").
        filename: String,
    },
}

// ============================================================================
// Target types (discriminated union on "kind")
// ============================================================================

/// Where to deliver the conversion result — discriminated union on `kind`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Target {
    /// Return results in the response body (default).
    #[serde(rename = "inbody")]
    InBody,

    /// Return results as a ZIP archive.
    #[serde(rename = "zip")]
    Zip,
}

impl Default for Target {
    fn default() -> Self {
        Target::InBody
    }
}

// ============================================================================
// Request options
// ============================================================================

/// Options for document conversion. All fields are optional — the server
/// applies sensible defaults for any omitted field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConvertDocumentsRequestOptions {
    /// Input format(s) to convert from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_formats: Option<Vec<InputFormat>>,

    /// Output format(s) to convert to. Defaults to `["md"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_formats: Option<Vec<OutputFormat>>,

    /// Image export mode. Defaults to `embedded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_export_mode: Option<ImageRefMode>,

    /// Enable OCR processing. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_ocr: Option<bool>,

    /// Force OCR over existing text. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_ocr: Option<bool>,

    /// OCR engine to use. Defaults to `easyocr`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_engine: Option<OcrEngine>,

    /// Languages for the OCR engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_lang: Option<Vec<String>>,

    /// PDF parsing backend. Defaults to `dlparse_v4`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_backend: Option<PdfBackend>,

    /// Table structure extraction mode. Defaults to `accurate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mode: Option<TableFormerMode>,

    /// Match table cell predictions back to PDF cells. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_matching: Option<bool>,

    /// Processing pipeline. Defaults to `standard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<ProcessingPipeline>,

    /// Page range to convert `[start, end]`. Pages start at 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_range: Option<(i64, i64)>,

    /// Per-document processing timeout in seconds. Defaults to 604800.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_timeout: Option<f64>,

    /// Abort on error. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_on_error: Option<bool>,

    /// Extract table structure. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_table_structure: Option<bool>,

    /// Extract images from documents. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_images: Option<bool>,

    /// Scale factor for images. Defaults to 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images_scale: Option<f64>,

    /// Placeholder between pages in markdown output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_page_break_placeholder: Option<String>,

    /// Enable OCR code enrichment. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_code_enrichment: Option<bool>,

    /// Enable formula OCR (LaTeX). Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_formula_enrichment: Option<bool>,

    /// Enable picture classification. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_picture_classification: Option<bool>,

    /// Enable chart data extraction. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_chart_extraction: Option<bool>,

    /// Enable picture description. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_picture_description: Option<bool>,

    /// Minimum area percentage for picture processing. Defaults to 0.05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_description_area_threshold: Option<f64>,

    /// VLM pipeline model preset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlm_pipeline_model: Option<VlmModelType>,

    /// Options for running a local VLM for picture description.
    /// Pass as a JSON object. Mutually exclusive with `picture_description_api`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_description_local: Option<serde_json::Value>,

    /// API details for a VLM used for picture description.
    /// Pass as a JSON object. Mutually exclusive with `picture_description_local`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_description_api: Option<serde_json::Value>,

    /// Options for running a local VLM for the VLM pipeline.
    /// Pass as a JSON object. Mutually exclusive with `vlm_pipeline_model_api` and `vlm_pipeline_model`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlm_pipeline_model_local: Option<serde_json::Value>,

    /// API details for a VLM used in the VLM pipeline.
    /// Pass as a JSON object. Mutually exclusive with `vlm_pipeline_model_local` and `vlm_pipeline_model`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlm_pipeline_model_api: Option<serde_json::Value>,
}

// ============================================================================
// Request body
// ============================================================================

/// Request body for `POST /v1/convert/source` and `POST /v1/convert/source/async`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertDocumentsRequest {
    /// One or more document sources to convert.
    pub sources: Vec<Source>,

    /// Conversion options (all optional, server uses defaults).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ConvertDocumentsRequestOptions>,

    /// Where to deliver results. Defaults to in-body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}
