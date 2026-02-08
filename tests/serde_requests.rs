//! Serialization tests for request types: Source, Target, options, and full requests.

use serde_json::json;

use docling_rs::models::enums::*;
use docling_rs::models::requests::*;

// ============================================================================
// Source (discriminated union on "kind")
// ============================================================================

#[test]
fn source_http_serialization() {
    let source = Source::Http {
        url: "https://example.com/doc.pdf".to_string(),
        headers: None,
    };

    let json = serde_json::to_value(&source).unwrap();
    assert_eq!(json["kind"], "http");
    assert_eq!(json["url"], "https://example.com/doc.pdf");
    // headers should be absent (skip_serializing_if)
    assert!(json.get("headers").is_none());
}

#[test]
fn source_http_with_headers_round_trip() {
    let mut headers = std::collections::HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());

    let source = Source::Http {
        url: "https://example.com/doc.pdf".to_string(),
        headers: Some(headers),
    };

    let json_str = serde_json::to_string(&source).unwrap();
    let deserialized: Source = serde_json::from_str(&json_str).unwrap();

    match deserialized {
        Source::Http { url, headers } => {
            assert_eq!(url, "https://example.com/doc.pdf");
            assert_eq!(
                headers.unwrap().get("Authorization").unwrap(),
                "Bearer token123"
            );
        }
        _ => panic!("Expected Source::Http"),
    }
}

#[test]
fn source_file_serialization() {
    let source = Source::File {
        base64_string: "SGVsbG8gV29ybGQ=".to_string(),
        filename: "test.pdf".to_string(),
    };

    let json = serde_json::to_value(&source).unwrap();
    assert_eq!(json["kind"], "file");
    assert_eq!(json["base64_string"], "SGVsbG8gV29ybGQ=");
    assert_eq!(json["filename"], "test.pdf");
}

#[test]
fn source_deserialize_from_tagged_json() {
    // Simulates what the server would send or what we'd construct in JSON
    let json = json!({
        "kind": "http",
        "url": "https://arxiv.org/pdf/2206.01062"
    });

    let source: Source = serde_json::from_value(json).unwrap();
    match source {
        Source::Http { url, headers } => {
            assert_eq!(url, "https://arxiv.org/pdf/2206.01062");
            assert!(headers.is_none());
        }
        _ => panic!("Expected Source::Http"),
    }
}

// ============================================================================
// Target (discriminated union on "kind")
// ============================================================================

#[test]
fn target_inbody_serialization() {
    let target = Target::InBody;
    let json = serde_json::to_value(&target).unwrap();
    assert_eq!(json, json!({"kind": "inbody"}));
}

#[test]
fn target_zip_serialization() {
    let target = Target::Zip;
    let json = serde_json::to_value(&target).unwrap();
    assert_eq!(json, json!({"kind": "zip"}));
}

#[test]
fn target_default_is_inbody() {
    let target = Target::default();
    let json = serde_json::to_value(&target).unwrap();
    assert_eq!(json["kind"], "inbody");
}

#[test]
fn target_round_trip() {
    let json_str = r#"{"kind":"zip"}"#;
    let target: Target = serde_json::from_str(json_str).unwrap();
    match target {
        Target::Zip => {} // correct
        _ => panic!("Expected Target::Zip"),
    }
}

// ============================================================================
// ConvertDocumentsRequestOptions
// ============================================================================

#[test]
fn options_default_serializes_to_empty_object() {
    let opts = ConvertDocumentsRequestOptions::default();
    let json = serde_json::to_value(&opts).unwrap();
    assert_eq!(
        json,
        json!({}),
        "Default options should serialize to {{}} (all fields None)"
    );
}

#[test]
fn options_with_some_fields_set() {
    let opts = ConvertDocumentsRequestOptions {
        to_formats: Some(vec![OutputFormat::Md, OutputFormat::Text]),
        do_ocr: Some(true),
        ocr_engine: Some(OcrEngine::Easyocr),
        ..Default::default()
    };

    let json = serde_json::to_value(&opts).unwrap();

    // Set fields should be present
    assert_eq!(json["to_formats"], json!(["md", "text"]));
    assert_eq!(json["do_ocr"], json!(true));
    assert_eq!(json["ocr_engine"], json!("easyocr"));

    // Unset fields should be absent (not null)
    assert!(json.get("from_formats").is_none());
    assert!(json.get("force_ocr").is_none());
    assert!(json.get("pdf_backend").is_none());
}

#[test]
fn options_page_range_serializes_as_array() {
    let opts = ConvertDocumentsRequestOptions {
        page_range: Some((1, 5)),
        ..Default::default()
    };

    let json = serde_json::to_value(&opts).unwrap();
    assert_eq!(json["page_range"], json!([1, 5]));
}

#[test]
fn options_page_range_round_trip_large_value() {
    // The server default for page_range end is i64::MAX
    let json = json!({
        "page_range": [1, 9223372036854775807_i64]
    });

    let opts: ConvertDocumentsRequestOptions = serde_json::from_value(json).unwrap();
    assert_eq!(opts.page_range, Some((1, i64::MAX)));
}

// ============================================================================
// ConvertDocumentsRequest (full request body)
// ============================================================================

#[test]
fn full_request_serialization() {
    let request = ConvertDocumentsRequest {
        sources: vec![Source::Http {
            url: "https://example.com/doc.pdf".to_string(),
            headers: None,
        }],
        options: Some(ConvertDocumentsRequestOptions {
            to_formats: Some(vec![OutputFormat::Md]),
            ..Default::default()
        }),
        target: None,
    };

    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(json["sources"][0]["kind"], "http");
    assert_eq!(json["sources"][0]["url"], "https://example.com/doc.pdf");
    assert_eq!(json["options"]["to_formats"], json!(["md"]));
    // target is None -> should be absent
    assert!(json.get("target").is_none());
}

#[test]
fn request_with_target_zip() {
    let request = ConvertDocumentsRequest {
        sources: vec![Source::Http {
            url: "https://example.com/doc.pdf".to_string(),
            headers: None,
        }],
        options: None,
        target: Some(Target::Zip),
    };

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["target"]["kind"], "zip");
    // options is None -> absent
    assert!(json.get("options").is_none());
}

#[test]
fn request_empty_sources() {
    let request = ConvertDocumentsRequest {
        sources: vec![],
        options: None,
        target: None,
    };

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["sources"], json!([]));
}
