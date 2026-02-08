//! Mock tests for file upload endpoints (convert_file, convert_file_async).

mod common;

use std::io::Write;

#[tokio::test]
async fn convert_file_sends_multipart_and_parses_response() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/file")
        .match_header("content-type", mockito::Matcher::Regex(
            "multipart/form-data".to_string(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    // Create a temporary file
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.write_all(b"fake pdf content").unwrap();

    // Rename to .pdf so MIME guessing works
    let tmp_path = tmpfile.path().to_path_buf();

    let client = common::test_client(&server.url());
    let result = client
        .convert_file(&[tmp_path.to_str().unwrap()], None, None)
        .await
        .unwrap();

    assert_eq!(result.document.filename, "test.pdf");
    assert_eq!(result.processing_time, 1.234);
    mock.assert_async().await;
}

#[tokio::test]
async fn convert_file_with_options() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/file")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.write_all(b"fake pdf content").unwrap();
    let tmp_path = tmpfile.path().to_path_buf();

    let opts = docling_rs::ConvertDocumentsRequestOptions {
        to_formats: Some(vec![docling_rs::OutputFormat::Md]),
        do_ocr: Some(true),
        ..Default::default()
    };

    let client = common::test_client(&server.url());
    let result = client
        .convert_file(
            &[tmp_path.to_str().unwrap()],
            Some(&opts),
            Some(&docling_rs::TargetName::Inbody),
        )
        .await
        .unwrap();

    assert_eq!(result.document.filename, "test.pdf");
    mock.assert_async().await;
}

#[tokio::test]
async fn convert_file_async_returns_task_status() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/file/async")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("file-task-001", "PENDING")).unwrap(),
        )
        .create_async()
        .await;

    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.write_all(b"fake pdf content").unwrap();
    let tmp_path = tmpfile.path().to_path_buf();

    let client = common::test_client(&server.url());
    let task = client
        .convert_file_async(&[tmp_path.to_str().unwrap()], None, None)
        .await
        .unwrap();

    assert_eq!(task.task_id, "file-task-001");
    assert_eq!(task.task_status, "PENDING");
    mock.assert_async().await;
}

#[tokio::test]
async fn convert_file_nonexistent_path_returns_io_error() {
    // No server mock needed — the error happens before any HTTP call
    let client = common::test_client("http://127.0.0.1:9999");
    let result = client
        .convert_file(&["./definitely_does_not_exist.pdf"], None, None)
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        docling_rs::DoclingError::Io(_) => {} // correct
        other => panic!("Expected DoclingError::Io, got: {:?}", other),
    }
}

#[tokio::test]
async fn wait_for_file_conversion_happy_path() {
    let mut server = mockito::Server::new_async().await;

    // Step 1: async file submit
    let submit_mock = server
        .mock("POST", "/v1/convert/file/async")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("file-task-002", "PENDING")).unwrap(),
        )
        .create_async()
        .await;

    // Step 2: poll returns SUCCESS
    let poll_mock = server
        .mock("GET", mockito::Matcher::Regex(
            r"/v1/status/poll/file-task-002.*".to_string(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("file-task-002", "SUCCESS")).unwrap(),
        )
        .create_async()
        .await;

    // Step 3: get result
    let result_mock = server
        .mock("GET", "/v1/result/file-task-002")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.write_all(b"fake pdf content").unwrap();
    let tmp_path = tmpfile.path().to_path_buf();

    let client = common::test_client(&server.url());
    let result = client
        .wait_for_file_conversion(
            &[tmp_path.to_str().unwrap()],
            None,
            None,
            std::time::Duration::from_secs(30),
            Some(1.0),
        )
        .await
        .unwrap();

    assert_eq!(result.document.filename, "test.pdf");
    submit_mock.assert_async().await;
    poll_mock.assert_async().await;
    result_mock.assert_async().await;
}
