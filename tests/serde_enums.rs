//! Serialization round-trip tests for all API enums.
//!
//! Verifies that every enum variant serializes to the exact string the
//! OpenAPI spec defines, and can be deserialized back to the same variant.

use docling_rs::models::enums::*;

// ============================================================================
// Helper: test that serialize -> deserialize round-trips correctly
// ============================================================================

fn assert_enum_serializes_to<T>(value: &T, expected: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    // Serialize to JSON string (e.g. "\"snake_case\"")
    let json_str = serde_json::to_string(value).unwrap();
    assert_eq!(
        json_str,
        format!("\"{}\"", expected),
        "Serialization mismatch for {:?}",
        value
    );

    // Deserialize back
    let deserialized: T = serde_json::from_str(&json_str).unwrap();
    assert_eq!(&deserialized, value, "Round-trip mismatch for {:?}", value);
}

// ============================================================================
// InputFormat — 15 variants, 4 with custom renames
// ============================================================================

#[test]
fn input_format_standard_variants() {
    assert_enum_serializes_to(&InputFormat::Docx, "docx");
    assert_enum_serializes_to(&InputFormat::Pptx, "pptx");
    assert_enum_serializes_to(&InputFormat::Html, "html");
    assert_enum_serializes_to(&InputFormat::Image, "image");
    assert_enum_serializes_to(&InputFormat::Pdf, "pdf");
    assert_enum_serializes_to(&InputFormat::Asciidoc, "asciidoc");
    assert_enum_serializes_to(&InputFormat::Md, "md");
    assert_enum_serializes_to(&InputFormat::Csv, "csv");
    assert_enum_serializes_to(&InputFormat::Xlsx, "xlsx");
    assert_enum_serializes_to(&InputFormat::Audio, "audio");
    assert_enum_serializes_to(&InputFormat::Vtt, "vtt");
}

#[test]
fn input_format_custom_renamed_variants() {
    assert_enum_serializes_to(&InputFormat::XmlUspto, "xml_uspto");
    assert_enum_serializes_to(&InputFormat::XmlJats, "xml_jats");
    assert_enum_serializes_to(&InputFormat::MetsGbs, "mets_gbs");
    assert_enum_serializes_to(&InputFormat::JsonDocling, "json_docling");
}

// ============================================================================
// OutputFormat — 7 variants, 1 custom rename
// ============================================================================

#[test]
fn output_format_all_variants() {
    assert_enum_serializes_to(&OutputFormat::Md, "md");
    assert_enum_serializes_to(&OutputFormat::Json, "json");
    assert_enum_serializes_to(&OutputFormat::Yaml, "yaml");
    assert_enum_serializes_to(&OutputFormat::Html, "html");
    assert_enum_serializes_to(&OutputFormat::HtmlSplitPage, "html_split_page");
    assert_enum_serializes_to(&OutputFormat::Text, "text");
    assert_enum_serializes_to(&OutputFormat::Doctags, "doctags");
}

// ============================================================================
// ImageRefMode — 3 variants
// ============================================================================

#[test]
fn image_ref_mode_all_variants() {
    assert_enum_serializes_to(&ImageRefMode::Placeholder, "placeholder");
    assert_enum_serializes_to(&ImageRefMode::Embedded, "embedded");
    assert_enum_serializes_to(&ImageRefMode::Referenced, "referenced");
}

// ============================================================================
// TableFormerMode — 2 variants
// ============================================================================

#[test]
fn table_former_mode_all_variants() {
    assert_enum_serializes_to(&TableFormerMode::Fast, "fast");
    assert_enum_serializes_to(&TableFormerMode::Accurate, "accurate");
}

// ============================================================================
// PdfBackend — 4 variants, 3 custom renames
// ============================================================================

#[test]
fn pdf_backend_all_variants() {
    assert_enum_serializes_to(&PdfBackend::Pypdfium2, "pypdfium2");
    assert_enum_serializes_to(&PdfBackend::DlparseV1, "dlparse_v1");
    assert_enum_serializes_to(&PdfBackend::DlparseV2, "dlparse_v2");
    assert_enum_serializes_to(&PdfBackend::DlparseV4, "dlparse_v4");
}

// ============================================================================
// ProcessingPipeline — 4 variants
// ============================================================================

#[test]
fn processing_pipeline_all_variants() {
    assert_enum_serializes_to(&ProcessingPipeline::Legacy, "legacy");
    assert_enum_serializes_to(&ProcessingPipeline::Standard, "standard");
    assert_enum_serializes_to(&ProcessingPipeline::Vlm, "vlm");
    assert_enum_serializes_to(&ProcessingPipeline::Asr, "asr");
}

// ============================================================================
// OcrEngine — 6 variants
// ============================================================================

#[test]
fn ocr_engine_all_variants() {
    assert_enum_serializes_to(&OcrEngine::Auto, "auto");
    assert_enum_serializes_to(&OcrEngine::Easyocr, "easyocr");
    assert_enum_serializes_to(&OcrEngine::Ocrmac, "ocrmac");
    assert_enum_serializes_to(&OcrEngine::Rapidocr, "rapidocr");
    assert_enum_serializes_to(&OcrEngine::Tesserocr, "tesserocr");
    assert_enum_serializes_to(&OcrEngine::Tesseract, "tesseract");
}

// ============================================================================
// ConversionStatus — 6 variants
// ============================================================================

#[test]
fn conversion_status_all_variants() {
    assert_enum_serializes_to(&ConversionStatus::Pending, "pending");
    assert_enum_serializes_to(&ConversionStatus::Started, "started");
    assert_enum_serializes_to(&ConversionStatus::Failure, "failure");
    assert_enum_serializes_to(&ConversionStatus::Success, "success");
    assert_enum_serializes_to(&ConversionStatus::PartialSuccess, "partial_success");
    assert_enum_serializes_to(&ConversionStatus::Skipped, "skipped");
}

// ============================================================================
// DoclingComponentType — 5 variants
// ============================================================================

#[test]
fn docling_component_type_all_variants() {
    assert_enum_serializes_to(&DoclingComponentType::DocumentBackend, "document_backend");
    assert_enum_serializes_to(&DoclingComponentType::Model, "model");
    assert_enum_serializes_to(&DoclingComponentType::DocAssembler, "doc_assembler");
    assert_enum_serializes_to(&DoclingComponentType::UserInput, "user_input");
    assert_enum_serializes_to(&DoclingComponentType::Pipeline, "pipeline");
}

// ============================================================================
// ProfilingScope — 2 variants
// ============================================================================

#[test]
fn profiling_scope_all_variants() {
    assert_enum_serializes_to(&ProfilingScope::Page, "page");
    assert_enum_serializes_to(&ProfilingScope::Document, "document");
}

// ============================================================================
// TaskType — 2 variants
// ============================================================================

#[test]
fn task_type_all_variants() {
    assert_enum_serializes_to(&TaskType::Convert, "convert");
    assert_enum_serializes_to(&TaskType::Chunk, "chunk");
}

// ============================================================================
// VlmModelType — 9 variants, many custom renames
// ============================================================================

#[test]
fn vlm_model_type_all_variants() {
    assert_enum_serializes_to(&VlmModelType::Smoldocling, "smoldocling");
    assert_enum_serializes_to(&VlmModelType::SmoldoclingVllm, "smoldocling_vllm");
    assert_enum_serializes_to(&VlmModelType::GraniteVision, "granite_vision");
    assert_enum_serializes_to(&VlmModelType::GraniteVisionVllm, "granite_vision_vllm");
    assert_enum_serializes_to(&VlmModelType::GraniteVisionOllama, "granite_vision_ollama");
    assert_enum_serializes_to(&VlmModelType::GotOcr2, "got_ocr_2");
    assert_enum_serializes_to(&VlmModelType::GraniteDocling, "granite_docling");
    assert_enum_serializes_to(&VlmModelType::GraniteDoclingVllm, "granite_docling_vllm");
    assert_enum_serializes_to(&VlmModelType::DeepsekocrOllama, "deepseekocr_ollama");
}

// ============================================================================
// TargetName — 2 variants + Default + Display
// ============================================================================

#[test]
fn target_name_all_variants() {
    assert_enum_serializes_to(&TargetName::Inbody, "inbody");
    assert_enum_serializes_to(&TargetName::Zip, "zip");
}

#[test]
fn target_name_default_is_inbody() {
    assert_eq!(TargetName::default(), TargetName::Inbody);
}

#[test]
fn target_name_display_matches_serde() {
    assert_eq!(format!("{}", TargetName::Inbody), "inbody");
    assert_eq!(format!("{}", TargetName::Zip), "zip");
}

// ============================================================================
// Display impls — verify consistency with serde for all enums that have Display
// ============================================================================

#[test]
fn display_impls_match_serde() {
    // InputFormat
    assert_eq!(format!("{}", InputFormat::XmlUspto), "xml_uspto");
    assert_eq!(format!("{}", InputFormat::JsonDocling), "json_docling");

    // OutputFormat
    assert_eq!(
        format!("{}", OutputFormat::HtmlSplitPage),
        "html_split_page"
    );
    assert_eq!(format!("{}", OutputFormat::Md), "md");

    // ImageRefMode
    assert_eq!(format!("{}", ImageRefMode::Embedded), "embedded");

    // TableFormerMode
    assert_eq!(format!("{}", TableFormerMode::Accurate), "accurate");

    // PdfBackend
    assert_eq!(format!("{}", PdfBackend::DlparseV4), "dlparse_v4");

    // ProcessingPipeline
    assert_eq!(format!("{}", ProcessingPipeline::Vlm), "vlm");

    // OcrEngine
    assert_eq!(format!("{}", OcrEngine::Easyocr), "easyocr");

    // VlmModelType
    assert_eq!(format!("{}", VlmModelType::GotOcr2), "got_ocr_2");
    assert_eq!(
        format!("{}", VlmModelType::DeepsekocrOllama),
        "deepseekocr_ollama"
    );

    // ConversionStatus
    assert_eq!(format!("{}", ConversionStatus::Success), "success");
    assert_eq!(
        format!("{}", ConversionStatus::PartialSuccess),
        "partial_success"
    );
}
