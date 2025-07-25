use crate::settings::Settings;

pub fn merge_settings(cli: Settings, file_and_env: Settings) -> Settings {
    Settings {
        address: cli.address.or(file_and_env.address),
        device: cli.device.or(file_and_env.device),
        subtitles: cli.subtitles.or(file_and_env.subtitles),
        no_search: cli.no_search || file_and_env.no_search,
        loop_playback: cli.loop_playback || file_and_env.loop_playback,
        shuffle: cli.shuffle || file_and_env.shuffle,
        seek: cli.seek.or(file_and_env.seek),
        volume_step: cli.volume_step.or(file_and_env.volume_step),
        tomp4: cli.tomp4 || file_and_env.tomp4,
        media_type: cli.media_type.or(file_and_env.media_type),
        quiet: cli.quiet || file_and_env.quiet,
        no_metadata: cli.no_metadata || file_and_env.no_metadata,
        no_cover: cli.no_cover || file_and_env.no_cover,
        show_options: cli.show_options || file_and_env.show_options,
        exit: cli.exit || file_and_env.exit,
        command: cli.command.or(file_and_env.command),
        media_path: cli.media_path.or(file_and_env.media_path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Settings;

    #[test]
    fn test_merge_cli_overrides_file_and_env() {
        let cli_settings = Settings {
            address: Some("192.168.1.100".to_string()),
            device: None,
            subtitles: None,
            no_search: true,
            loop_playback: false,
            shuffle: false,
            seek: None,
            volume_step: Some(0.05),
            tomp4: false,
            media_type: None,
            quiet: false,
            no_metadata: false,
            no_cover: false,
            show_options: false,
            exit: false,
            command: None,
            media_path: None,
        };

        let file_and_env_settings = Settings {
            address: Some("192.168.1.101".to_string()),
            device: Some("Bedroom TV".to_string()),
            subtitles: Some("sub.srt".to_string()),
            no_search: false,
            loop_playback: true,
            shuffle: false,
            seek: None,
            volume_step: Some(0.1),
            tomp4: true,
            media_type: None,
            quiet: false,
            no_metadata: false,
            no_cover: false,
            show_options: false,
            exit: false,
            command: None,
            media_path: None,
        };

        let merged = merge_settings(cli_settings, file_and_env_settings);

        assert_eq!(merged.address, Some("192.168.1.100".to_string())); // CLI overrides
        assert_eq!(merged.device, Some("Bedroom TV".to_string())); // File/Env is present, CLI is None
        assert_eq!(merged.subtitles, Some("sub.srt".to_string()));
        assert_eq!(merged.no_search, true); // CLI overrides
        assert_eq!(merged.loop_playback, true); // File/Env is true, CLI is false
        assert_eq!(merged.volume_step, Some(0.05)); // CLI overrides
        assert_eq!(merged.tomp4, true); // File/Env is true, CLI is false
    }

    #[test]
    fn test_merge_file_and_env_when_cli_none() {
        let cli_settings = Settings {
            address: None,
            device: None,
            subtitles: None,
            no_search: false,
            loop_playback: false,
            shuffle: false,
            seek: None,
            volume_step: None,
            tomp4: false,
            media_type: None,
            quiet: false,
            no_metadata: false,
            no_cover: false,
            show_options: false,
            exit: false,
            command: None,
            media_path: None,
        };

        let file_and_env_settings = Settings {
            address: Some("192.168.1.101".to_string()),
            device: Some("Bedroom TV".to_string()),
            subtitles: Some("sub.srt".to_string()),
            no_search: true,
            loop_playback: true,
            shuffle: true,
            seek: Some("10s".to_string()),
            volume_step: Some(0.1),
            tomp4: true,
            media_type: Some("video/mp4".to_string()),
            quiet: true,
            no_metadata: true,
            no_cover: true,
            show_options: true,
            exit: true,
            command: Some("play".to_string()),
            media_path: Some("file.mp4".to_string()),
        };

        let merged = merge_settings(cli_settings, file_and_env_settings);

        assert_eq!(merged.address, Some("192.168.1.101".to_string()));
        assert_eq!(merged.device, Some("Bedroom TV".to_string()));
        assert_eq!(merged.subtitles, Some("sub.srt".to_string()));
        assert_eq!(merged.no_search, true);
        assert_eq!(merged.loop_playback, true);
        assert_eq!(merged.shuffle, true);
        assert_eq!(merged.seek, Some("10s".to_string()));
        assert_eq!(merged.volume_step, Some(0.1));
        assert_eq!(merged.tomp4, true);
        assert_eq!(merged.media_type, Some("video/mp4".to_string()));
        assert_eq!(merged.quiet, true);
        assert_eq!(merged.no_metadata, true);
        assert_eq!(merged.no_cover, true);
        assert_eq!(merged.show_options, true);
        assert_eq!(merged.exit, true);
        assert_eq!(merged.command, Some("play".to_string()));
        assert_eq!(merged.media_path, Some("file.mp4".to_string()));
    }
}
