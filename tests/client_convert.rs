//! Mock tests for synchronous URL conversion and error handling.

mod common;

use serde_json::json;

#[tokio::test]
async fn convert_source_sends_correct_body_and_parses_response() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/source")
        .match_header("content-type", mockito::Matcher::Regex(
            "application/json".to_string(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let result = client
        .convert_source("https://arxiv.org/pdf/2206.01062", None)
        .await
        .unwrap();

    assert_eq!(result.document.filename, "test.pdf");
    assert_eq!(
        result.document.md_content.as_deref(),
        Some("# Hello World\n\nThis is a test document.")
    );
    assert_eq!(result.processing_time, 1.234);
    mock.assert_async().await;
}

#[tokio::test]
async fn convert_source_with_options() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/source")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let opts = docling_rs::ConvertDocumentsRequestOptions {
        to_formats: Some(vec![
            docling_rs::OutputFormat::Md,
            docling_rs::OutputFormat::Text,
        ]),
        do_ocr: Some(true),
        ..Default::default()
    };

    let result = client
        .convert_source("https://example.com/doc.pdf", Some(opts))
        .await
        .unwrap();

    assert_eq!(result.document.filename, "test.pdf");
    mock.assert_async().await;
}

#[tokio::test]
async fn convert_full_request() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/source")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client(&server.url());

    let request = docling_rs::ConvertDocumentsRequest {
        sources: vec![docling_rs::Source::Http {
            url: "https://example.com/doc.pdf".to_string(),
            headers: None,
        }],
        options: None,
        target: Some(docling_rs::Target::InBody),
    };

    let result = client.convert(&request).await.unwrap();
    assert_eq!(result.document.filename, "test.pdf");
    mock.assert_async().await;
}

#[tokio::test]
async fn convert_source_422_returns_api_error() {
    let mut server = mockito::Server::new_async().await;

    let error_body = json!({
        "detail": [{
            "loc": ["body", "sources"],
            "msg": "field required",
            "type": "missing"
        }]
    });

    let mock = server
        .mock("POST", "/v1/convert/source")
        .with_status(422)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&error_body).unwrap())
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let result = client
        .convert_source("https://example.com/doc.pdf", None)
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        docling_rs::DoclingError::Api { status_code, body } => {
            assert_eq!(status_code, 422);
            assert!(body.contains("field required"));
        }
        other => panic!("Expected DoclingError::Api, got: {:?}", other),
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn convert_source_500_empty_body() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/source")
        .with_status(500)
        .with_body("")
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let result = client
        .convert_source("https://example.com/doc.pdf", None)
        .await;

    match result.unwrap_err() {
        docling_rs::DoclingError::Api { status_code, body } => {
            assert_eq!(status_code, 500);
            assert_eq!(body, "");
        }
        other => panic!("Expected DoclingError::Api, got: {:?}", other),
    }
    mock.assert_async().await;
}
