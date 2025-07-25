use clap::{Parser, Subcommand};
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use http_body_util::{Full};
use bytes::Bytes;
use hyper_util::rt::TokioIo;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Play a local video file
    PlayFile {
        /// Path to the video file
        #[arg(short, long)]
        file: String,
    },
    /// Play media files from a local directory
    PlayDir {
        /// Path to the directory
        #[arg(short, long)]
        dir: String,
    },
    /// Play multiple video files continuously
    PlayMultiple {
        /// Paths to the video files
        #[arg(short, long, num_args = 1..)]
        files: Vec<String>,
    },
    /// Play an MP4 file from a URL
    PlayUrl {
        /// URL of the MP4 file
        #[arg(short, long)]
        url: String,
    },
    /// Play a video from a Torrent file or magnet link
    PlayTorrent {
        /// Path to the Torrent file or magnet link
        #[arg(short, long)]
        torrent: String,
    },
}

async fn serve_file(req: Request<hyper::body::Incoming>, file_path: Arc<PathBuf>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    if req.uri().path() != "/" {
        let mut not_found = Response::new(Full::new(Bytes::from("Not Found")));
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        return Ok(not_found);
    }

    let mut file = match File::open(&*file_path).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            let mut err_response = Response::new(Full::new(Bytes::from("Internal Server Error")));
            *err_response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(err_response);
        }
    };

    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer).await {
        eprintln!("Error reading file: {}", e);
        let mut err_response = Response::new(Full::new(Bytes::from("Internal Server Error")));
        *err_response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        return Ok(err_response);
    }

    Ok(Response::new(Full::new(Bytes::from(buffer))))
}

pub async fn start_server(file_to_serve: PathBuf, shutdown_rx: tokio::sync::oneshot::Receiver<()>) -> Result<(SocketAddr, tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // Use 0 to let the OS choose a free port
    let listener = TcpListener::bind(addr).await?;
    let local_addr = listener.local_addr()?;
    println!("Listening on http://{}", local_addr);

    let file_path_arc = Arc::new(file_to_serve);

    let server_handle = tokio::spawn(async move {
        tokio::select! {
            _ = shutdown_rx => {
                println!("Server shutting down.");
            }
            _ = async {
                loop {
                    let (stream, _) = listener.accept().await?;
                    let io = TokioIo::new(stream);
                    let file_path_clone = file_path_arc.clone();
                    tokio::task::spawn(async move {
                        if let Err(err) = http1::Builder::new()
                            .serve_connection(io, service_fn(move |req| serve_file(req, file_path_clone.clone())))
                            .await
                        {
                            eprintln!("Error serving connection: {:?}", err);
                        }
                    });
                }
                #[allow(unreachable_code)]
                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(()) // This line is unreachable but needed for type inference
            } => {}
        }
        Ok(())
    });

    Ok((local_addr, server_handle))
}