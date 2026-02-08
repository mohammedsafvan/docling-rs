//! Data models for the Docling Serve API.
//!
//! This module contains all request/response types and enums matching the
//! OpenAPI 3.1 specification (v1.12.0). It is organized into submodules:
//!
//! - [`enums`] — All API enums (InputFormat, OutputFormat, ConversionStatus, etc.)
//! - [`requests`] — Request types (Source, Target, ConvertDocumentsRequest, options)
//! - [`responses`] — Response types (ConvertDocumentResponse, TaskStatusResponse, etc.)

pub mod enums;
pub mod requests;
pub mod responses;

pub use enums::*;
pub use requests::*;
pub use responses::*;
