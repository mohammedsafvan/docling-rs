//! # docling_rs
//!
//! Rust SDK for [Docling Serve](https://github.com/docling-project/docling-serve) that makes document conversion simple, reliable, and production-ready in Rust.
//!
//! ## Modules
//!
//! - [`client`] — The async [`DoclingClient`] for interacting with Docling Serve.
#![cfg_attr(
    feature = "blocking",
    doc = " - [`blocking`] — Synchronous/blocking versions of all APIs."
)]
//! - [`error`] — The [`DoclingError`] type covering all failure modes.
//! - [`models`] — All request/response types and enums matching the OpenAPI spec.

pub mod client;
pub mod error;
pub mod models;

#[cfg(feature = "blocking")]
pub mod blocking;

// -- Primary types (always needed) --
pub use client::DoclingClient;
pub use error::DoclingError;

// -- Request types --
pub use models::requests::{ConvertDocumentsRequest, ConvertDocumentsRequestOptions, Source, Target};

// -- Response types --
pub use models::responses::{
    ConvertDocumentResponse, ExportDocumentResponse, HealthCheckResponse, TaskStatusResponse,
};

// -- Commonly used enums --
pub use models::enums::{
    ConversionStatus, InputFormat, OcrEngine, OutputFormat, PdfBackend, ProcessingPipeline,
    TargetName,
};
