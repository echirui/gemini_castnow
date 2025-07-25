use bytes::Bytes;
use gemini_castnow::start_server;
use http_body_util::{BodyExt, Empty};
use hyper::{Method, Request, Uri};
use hyper_util::client::legacy::{Client, connect::HttpConnector};
use hyper_util::rt::TokioExecutor;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

#[tokio::test]
async fn test_start_server() {
    // Create a dummy file to serve
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    tokio::fs::write(&file_path, b"Hello, world!")
        .await
        .unwrap();

    // Create a channel for server shutdown
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn the server in a separate task and get its address and handle
    let server_file_path = file_path.clone();
    let (server_addr, server_handle) = start_server(server_file_path, rx).await.unwrap();

    // Make a request to the server and verify the content
    let client = Client::builder(TokioExecutor::new()).build(HttpConnector::new());
    let uri = format!("http://{}/", server_addr).parse::<Uri>().unwrap();

    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Empty::<Bytes>::new())
        .unwrap();

    let response = client.request(request).await.unwrap();

    assert_eq!(response.status(), hyper::StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, "Hello, world!");

    // Send shutdown signal to the server
    let _ = tx.send(());

    // Await the server task to ensure it shuts down cleanly
    server_handle.await.unwrap().unwrap();

    // Clean up the temporary directory
    temp_dir.close().unwrap();
}
