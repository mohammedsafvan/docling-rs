use serde::{Deserialize, Serialize};

/// A document format supported by document backend parsers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InputFormat {
    Docx,
    Pptx,
    Html,
    Image,
    Pdf,
    Asciidoc,
    Md,
    Csv,
    Xlsx,
    #[serde(rename = "xml_uspto")]
    XmlUspto,
    #[serde(rename = "xml_jats")]
    XmlJats,
    #[serde(rename = "mets_gbs")]
    MetsGbs,
    #[serde(rename = "json_docling")]
    JsonDocling,
    Audio,
    Vtt,
}

impl std::fmt::Display for InputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// Output format for document conversion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Md,
    Json,
    Yaml,
    Html,
    #[serde(rename = "html_split_page")]
    HtmlSplitPage,
    Text,
    Doctags,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// Image export mode for the document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImageRefMode {
    Placeholder,
    Embedded,
    Referenced,
}

impl std::fmt::Display for ImageRefMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// Table structure extraction mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TableFormerMode {
    Fast,
    Accurate,
}

impl std::fmt::Display for TableFormerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// Available PDF parsing backends.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PdfBackend {
    Pypdfium2,
    #[serde(rename = "dlparse_v1")]
    DlparseV1,
    #[serde(rename = "dlparse_v2")]
    DlparseV2,
    #[serde(rename = "dlparse_v4")]
    DlparseV4,
}

impl std::fmt::Display for PdfBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// Available document processing pipeline types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingPipeline {
    Legacy,
    Standard,
    Vlm,
    Asr,
}

impl std::fmt::Display for ProcessingPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// OCR engine options.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OcrEngine {
    Auto,
    Easyocr,
    Ocrmac,
    Rapidocr,
    Tesserocr,
    Tesseract,
}

impl std::fmt::Display for OcrEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// Status of a document conversion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConversionStatus {
    Pending,
    Started,
    Failure,
    Success,
    PartialSuccess,
    Skipped,
}

impl std::fmt::Display for ConversionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// Docling component types (for error reporting).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DoclingComponentType {
    DocumentBackend,
    Model,
    DocAssembler,
    UserInput,
    Pipeline,
}

/// Profiling scope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProfilingScope {
    Page,
    Document,
}

/// Async task type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Convert,
    Chunk,
}

/// VLM model type presets.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VlmModelType {
    Smoldocling,
    #[serde(rename = "smoldocling_vllm")]
    SmoldoclingVllm,
    #[serde(rename = "granite_vision")]
    GraniteVision,
    #[serde(rename = "granite_vision_vllm")]
    GraniteVisionVllm,
    #[serde(rename = "granite_vision_ollama")]
    GraniteVisionOllama,
    #[serde(rename = "got_ocr_2")]
    GotOcr2,
    #[serde(rename = "granite_docling")]
    GraniteDocling,
    #[serde(rename = "granite_docling_vllm")]
    GraniteDoclingVllm,
    #[serde(rename = "deepseekocr_ollama")]
    DeepsekocrOllama,
}

impl std::fmt::Display for VlmModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self).unwrap();
        write!(f, "{}", s.as_str().unwrap())
    }
}

/// Flat string enum for the target type in multipart form requests.
///
/// Used as a simple string form field in `/v1/convert/file` (multipart),
/// as opposed to the tagged `Target` union used in JSON request bodies.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TargetName {
    Inbody,
    Zip,
}

impl Default for TargetName {
    fn default() -> Self {
        TargetName::Inbody
    }
}

impl std::fmt::Display for TargetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetName::Inbody => write!(f, "inbody"),
            TargetName::Zip => write!(f, "zip"),
        }
    }
}
