use std::{env, path::{Path, PathBuf}, process::Command};

use crate::{consts::FFMPEG_PATH, utils::cmd::is_command_available};


pub struct FfmpegService {
    ffmpeg_path: PathBuf
}


pub fn resolve_ffmpeg_path() -> PathBuf {
    let current_path = env::current_exe().unwrap();
    let parent_dir = current_path.parent().unwrap();
    let ffmpeg_path = parent_dir.join(FFMPEG_PATH);
    ffmpeg_path
}


impl FfmpegService {
    pub fn new() -> Self {
        log::info!("Is available ffmpeg: {}", is_command_available("ffmpeg", "-version"));
        log::info!("Is available ffprobe: {}", is_command_available("ffprobe", "-version"));
        log::info!("ffmpeg path: {}", resolve_ffmpeg_path().display());
        Self {
            ffmpeg_path: resolve_ffmpeg_path(),
        }
    }

    pub fn generate_thumbnail(&self, video_file_path: &Path, output_path: &Path) -> Result<(), String> {
        let status = Command::new("ffmpeg")
            .args(&[
                "-i",
                video_file_path.to_str().ok_or("Invalid file path")?,
                "-ss",
                "00:00:05", // Extract at the 5-second mark
                "-vframes",
                "1", // Capture a single frame
                "-vf",
                "scale=356:200",
                "-q:v",
                "2", // Quality level (lower is better)
                output_path.to_str().ok_or("Invalid output path")?,
            ])
            .status()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        if !status.success() {
            return Err("ffmpeg command failed".to_string());
        }

        Ok(())
    }

}
