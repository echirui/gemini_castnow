use clap::{Parser, Subcommand};

pub mod server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Play a local file
    PlayFile { file: String },
    /// Play media from a directory
    PlayDir { dir: String },
    /// Play multiple files
    PlayMultiple { files: Vec<String> },
    /// Play a URL
    PlayUrl { url: String },
    /// Play a torrent
    PlayTorrent { torrent: String },
}
