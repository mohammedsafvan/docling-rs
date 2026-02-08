//! Mock tests for health() and version() endpoints.

mod common;

use mockito::Matcher;

#[tokio::test]
async fn health_returns_ok() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/health")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::health_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let health = client.health().await.unwrap();

    assert_eq!(health.status, "ok");
    mock.assert_async().await;
}

#[tokio::test]
async fn health_server_error_returns_api_error() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/health")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let result = client.health().await;

    assert!(result.is_err());
    match result.unwrap_err() {
        docling_rs::DoclingError::Api { status_code, body } => {
            assert_eq!(status_code, 500);
            assert_eq!(body, "Internal Server Error");
        }
        other => panic!("Expected DoclingError::Api, got: {:?}", other),
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn version_returns_hashmap() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/version")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&common::version_response_json()).unwrap())
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    let version = client.version().await.unwrap();

    assert_eq!(version["version"], "1.12.0");
    assert_eq!(version["docling"], "2.31.0");
    mock.assert_async().await;
}

#[tokio::test]
async fn health_does_not_send_auth_header() {
    let mut server = mockito::Server::new_async().await;

    // Verify NO authorization header is sent when using new() (not with_api_key)
    let mock = server
        .mock("GET", "/health")
        .match_header("authorization", Matcher::Missing)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"status":"ok"}"#)
        .create_async()
        .await;

    let client = common::test_client(&server.url());
    client.health().await.unwrap();

    mock.assert_async().await;
}
