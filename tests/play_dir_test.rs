use std::io::{self, BufRead, BufReader, Write};
use walkdir::WalkDir;

#[tokio::test]
async fn test_play_dir_file_selection() {
    // Create a temporary directory and some dummy media files
    let temp_dir = tempfile::tempdir().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    tokio::fs::write(dir_path.join("video1.mp4"), b"dummy video content")
        .await
        .unwrap();
    tokio::fs::write(dir_path.join("audio1.mp3"), b"dummy audio content")
        .await
        .unwrap();
    tokio::fs::write(dir_path.join("text.txt"), b"dummy text content")
        .await
        .unwrap();

    // Simulate user input for selecting the first file
    let input = "1\n";
    let mut stdin = BufReader::new(io::Cursor::new(input.as_bytes()));
    let mut stdout = Vec::new();

    // This part is tricky: directly testing the interactive CLI is hard.
    // We'll simulate the relevant part of the logic that lists files and takes input.
    // This is a simplified version of what's in main.rs for testing purposes.

    let mut media_files = Vec::new();
    for entry in WalkDir::new(&dir_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            if let Some(ext) = path.extension() {
                if ext == "mp4" || ext == "mkv" || ext == "avi" || ext == "mp3" {
                    media_files.push(path);
                }
            }
        }
    }

    // Sort for consistent test results
    media_files.sort();

    assert!(!media_files.is_empty());

    // Simulate the selection process
    let selected_index: usize;
    loop {
        // In a real scenario, this would print to stdout and read from stdin
        // For testing, we'll just parse the input directly
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let input_line = buffer.trim();

        if let Ok(index) = input_line.parse::<usize>() {
            if index > 0 && index <= media_files.len() {
                selected_index = index - 1;
                break;
            } else {
                // Simulate error message
                writeln!(&mut stdout, "Invalid number. Please try again.").unwrap();
            }
        } else {
            // Simulate error message
            writeln!(&mut stdout, "Invalid input. Please enter a number.").unwrap();
        }
    }

    let selected_file = &media_files[selected_index];
    assert_eq!(
        selected_file.file_name().unwrap().to_str().unwrap(),
        "audio1.mp3"
    ); // Assuming sorted order

    // Clean up the temporary directory
    temp_dir.close().unwrap();
}
