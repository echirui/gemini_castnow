use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Settings {
    /// Address of the Chromecast
    #[arg(long)]
    pub address: Option<String>,

    /// Name of the Chromecast
    #[arg(long, short)]
    pub device: Option<String>,

    /// Path to subtitles file
    #[arg(long)]
    pub subtitles: Option<String>,

    /// Disable the search for Chromecast devices
    #[arg(long)]
    #[serde(default)]
    pub no_search: bool,

    /// Play in loop
    #[arg(long)]
    #[serde(default)]
    pub loop_playback: bool,

    /// Play on shuffle
    #[arg(long)]
    #[serde(default)]
    pub shuffle: bool,

    /// Start playing at a specific time
    #[arg(long)]
    pub seek: Option<String>,

    /// Set the volume step
    #[arg(long)]
    pub volume_step: Option<f32>,

    /// Transcode to mp4
    #[arg(long)]
    #[serde(default)]
    pub tomp4: bool,

    /// Set the MIME type
    #[arg(long, name = "type")]
    pub media_type: Option<String>,

    /// Disable the timeline
    #[arg(long, short)]
    #[serde(default)]
    pub quiet: bool,

    /// Disable the metadata
    #[arg(long)]
    #[serde(default)]
    pub no_metadata: bool,

    /// Disable the cover
    #[arg(long)]
    #[serde(default)]
    pub no_cover: bool,

    /// Show the options
    #[arg(long)]
    #[serde(default)]
    pub show_options: bool,

    /// Exit when playback starts
    #[arg(long)]
    #[serde(default)]
    pub exit: bool,

    /// Command to execute on the Chromecast
    #[arg(long)]
    pub command: Option<String>,

    pub media_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::parse_from(vec!["gemini_castnow"]);
        assert_eq!(settings.no_search, false);
        assert_eq!(settings.loop_playback, false);
        assert_eq!(settings.shuffle, false);
        assert_eq!(settings.volume_step, None);
        assert_eq!(settings.tomp4, false);
        assert_eq!(settings.quiet, false);
        assert_eq!(settings.no_metadata, false);
        assert_eq!(settings.no_cover, false);
        assert_eq!(settings.show_options, false);
        assert_eq!(settings.exit, false);
    }

    #[test]
    fn test_cli_override() {
        let settings = Settings::parse_from(vec![
            "gemini_castnow",
            "--no-search",
            "--loop-playback",
            "--volume-step",
            "0.1",
        ]);
        assert_eq!(settings.no_search, true);
        assert_eq!(settings.loop_playback, true);
        assert_eq!(settings.volume_step, Some(0.1));
    }

    #[test]
    fn test_deserialize_settings() {
        let json = r#"{
            "no_search": true,
            "volume_step": 0.2
        }"#;
        let settings: Settings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.no_search, true);
        assert_eq!(settings.volume_step, Some(0.2));
        assert_eq!(settings.loop_playback, false); // Default from serde
    }
}
