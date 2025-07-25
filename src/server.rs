use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::header::CONTENT_TYPE;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::sync::oneshot;

async fn handle_request(
    req: Request<Incoming>,
    file_path: PathBuf,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri().path();
    if path == "/" {
        let file = File::open(&file_path).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let mut response = Response::new(Full::new(Bytes::from(buffer)));
        response
            .headers_mut()
            .insert(CONTENT_TYPE, "video/mp4".parse().unwrap());
        *response.status_mut() = StatusCode::OK;
        Ok(response)
    } else {
        let mut not_found = Response::new(Full::new(Bytes::from("Not Found")));
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        Ok(not_found)
    }
}

pub async fn start_server(
    file_path: PathBuf,
    shutdown_rx: oneshot::Receiver<()>,
) -> anyhow::Result<(SocketAddr, tokio::task::JoinHandle<()>)> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let local_addr = listener.local_addr()?;

    let server_handle = tokio::spawn(async move {
        let mut shutdown_rx = shutdown_rx;
        loop {
            tokio::select! {
                res = listener.accept() => {
                    if let Ok((stream, _)) = res {
                        let file_path = file_path.clone();
                        let service = service_fn(move |req| handle_request(req, file_path.clone()));
                        let io = TokioIo::new(stream);
                        tokio::spawn(async move {
                            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                                eprintln!("server error: {err}");
                            }
                        });
                    }
                }
                _ = &mut shutdown_rx => {
                    break;
                }
            }
        }
    });

    Ok((local_addr, server_handle))
}
