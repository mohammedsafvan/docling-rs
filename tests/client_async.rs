//! Mock tests for async conversion lifecycle: submit, poll, result, wait_for_conversion.

mod common;

use std::time::Duration;

#[tokio::test]
async fn convert_source_async_returns_task_status() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/source/async")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-001", "PENDING")).unwrap(),
        )
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let task = client
        .convert_source_async("https://example.com/doc.pdf", None)
        .await
        .unwrap();

    assert_eq!(task.task_id, "task-001");
    assert_eq!(task.task_status, "PENDING");
    mock.assert_async().await;
}

#[tokio::test]
async fn poll_task_status_with_wait_param() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/v1/status/poll/task-002?wait=5")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-002", "STARTED")).unwrap(),
        )
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let status = client.poll_task_status("task-002", Some(5.0)).await.unwrap();

    assert_eq!(status.task_id, "task-002");
    assert_eq!(status.task_status, "STARTED");
    mock.assert_async().await;
}

#[tokio::test]
async fn poll_task_status_without_wait_param() {
    let mut server = mockito::Server::new_async().await;

    // No query string when wait_secs is None
    let mock = server
        .mock("GET", "/v1/status/poll/task-003")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-003", "SUCCESS")).unwrap(),
        )
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let status = client.poll_task_status("task-003", None).await.unwrap();

    assert_eq!(status.task_status, "SUCCESS");
    mock.assert_async().await;
}

#[tokio::test]
async fn get_task_result_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/v1/result/task-004")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let result = client.get_task_result("task-004").await.unwrap();

    assert_eq!(result.document.filename, "test.pdf");
    assert_eq!(result.processing_time, 1.234);
    mock.assert_async().await;
}

#[tokio::test]
async fn wait_for_conversion_happy_path() {
    let mut server = mockito::Server::new_async().await;

    // Step 1: async submit returns task-005
    let submit_mock = server
        .mock("POST", "/v1/convert/source/async")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-005", "PENDING")).unwrap(),
        )
        .create_async()
        .await;

    // Step 2: first poll returns SUCCESS
    let poll_mock = server
        .mock("GET", mockito::Matcher::Regex(
            r"/v1/status/poll/task-005.*".to_string(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-005", "SUCCESS")).unwrap(),
        )
        .create_async()
        .await;

    // Step 3: get result
    let result_mock = server
        .mock("GET", "/v1/result/task-005")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let result = client
        .wait_for_conversion(
            "https://example.com/doc.pdf",
            None,
            Duration::from_secs(30),
            Some(1.0),
        )
        .await
        .unwrap();

    assert_eq!(result.document.filename, "test.pdf");
    submit_mock.assert_async().await;
    poll_mock.assert_async().await;
    result_mock.assert_async().await;
}

#[tokio::test]
async fn wait_for_conversion_task_failure() {
    let mut server = mockito::Server::new_async().await;

    // Submit
    let submit_mock = server
        .mock("POST", "/v1/convert/source/async")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-fail", "PENDING")).unwrap(),
        )
        .create_async()
        .await;

    // Poll returns FAILURE
    let poll_mock = server
        .mock("GET", mockito::Matcher::Regex(
            r"/v1/status/poll/task-fail.*".to_string(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-fail", "FAILURE")).unwrap(),
        )
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let result = client
        .wait_for_conversion(
            "https://example.com/doc.pdf",
            None,
            Duration::from_secs(30),
            Some(1.0),
        )
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        docling_rs::DoclingError::TaskFailed { task_id, status } => {
            assert_eq!(task_id, "task-fail");
            assert_eq!(status, "FAILURE");
        }
        other => panic!("Expected TaskFailed, got: {:?}", other),
    }
    submit_mock.assert_async().await;
    poll_mock.assert_async().await;
}
