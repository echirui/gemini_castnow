use bytes::Bytes;
use http_body_util::Full;
use hyper::StatusCode;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use std::path::PathBuf;

#[tokio::test]
async fn test_start_server() {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let file_path = PathBuf::from("test_media.mp4");
    tokio::fs::File::create(&file_path).await.unwrap();

    let (addr, handle) = crate::server::start_server(file_path.clone(), rx)
        .await
        .unwrap();

    let connector = HttpConnector::new();
    let client: Client<HttpConnector, Full<Bytes>> =
        Client::builder(TokioExecutor::new()).build(connector);
    let uri = format!("http://{addr}/").parse().unwrap();
    let response = client.get(uri).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    tx.send(()).unwrap();
    handle.await.unwrap();

    tokio::fs::remove_file(&file_path).await.unwrap();
}
