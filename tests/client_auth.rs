//! Tests that Bearer token authentication is correctly applied (or absent).

mod common;

use mockito::Matcher;

/// Health/version are NOT secured endpoints per the OpenAPI spec, so even when
/// an API key is configured the client must NOT send an Authorization header.
#[tokio::test]
async fn health_does_not_send_auth_even_with_api_key() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/health")
        .match_header("authorization", Matcher::Missing)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"status":"ok"}"#)
        .create_async()
        .await;

    let client = common::test_client_with_key(&server.url(), "my-secret-key");
    let health = client.health().await.unwrap();

    assert_eq!(health.status, "ok");
    mock.assert_async().await;
}

#[tokio::test]
async fn without_api_key_no_auth_header() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/health")
        .match_header("authorization", Matcher::Missing)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"status":"ok"}"#)
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let health = client.health().await.unwrap();

    assert_eq!(health.status, "ok");
    mock.assert_async().await;
}

#[tokio::test]
async fn api_key_sent_on_convert_endpoint() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/source")
        .match_header("authorization", "Bearer api-token-xyz")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client_with_key(&server.url(), "api-token-xyz");
    let result = client
        .convert_source("https://example.com/doc.pdf", None)
        .await
        .unwrap();

    assert_eq!(result.document.filename, "test.pdf");
    mock.assert_async().await;
}

#[tokio::test]
async fn api_key_sent_on_async_endpoint() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/convert/source/async")
        .match_header("authorization", "Bearer secret-456")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-auth", "PENDING")).unwrap(),
        )
        .create_async()
        .await;

    let client = common::test_client_with_key(&server.url(), "secret-456");
    let task = client
        .convert_source_async("https://example.com/doc.pdf", None)
        .await
        .unwrap();

    assert_eq!(task.task_id, "task-auth");
    mock.assert_async().await;
}

#[tokio::test]
async fn api_key_sent_on_poll_and_result_endpoints() {
    let mut server = mockito::Server::new_async().await;

    let poll_mock = server
        .mock("GET", mockito::Matcher::Regex(
            r"/v1/status/poll/task-x.*".to_string(),
        ))
        .match_header("authorization", "Bearer key-789")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&common::task_status_json("task-x", "SUCCESS")).unwrap(),
        )
        .create_async()
        .await;

    let result_mock = server
        .mock("GET", "/v1/result/task-x")
        .match_header("authorization", "Bearer key-789")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::convert_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client_with_key(&server.url(), "key-789");

    let status = client.poll_task_status("task-x", Some(1.0)).await.unwrap();
    assert_eq!(status.task_status, "SUCCESS");

    let result = client.get_task_result("task-x").await.unwrap();
    assert_eq!(result.document.filename, "test.pdf");

    poll_mock.assert_async().await;
    result_mock.assert_async().await;
}
