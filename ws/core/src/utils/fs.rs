use std::{path::Path, process::Command};

use crate::{consts::{ACCEPTABLE_AUDIO_FORMATS, ACCEPTABLE_VIDEO_FORMATS}, models::MediaType};

pub fn check_file_exists(file_path: &String) -> bool {
  Path::new(&file_path).exists()
}

pub fn change_file_name_in_path(original_path: &Path, new_file_name: &str) -> Option<String> {
  let path = Path::new(original_path);
  let parent = path.parent()?;
  let new_path = parent.join(new_file_name);
  new_path.to_str().map(|s| s.to_string())
}

pub fn path_to_string(path: &Path) -> String {
  path.to_str()
      .expect("Failed to convert String to Path")
      .to_string()
}

pub fn extract_file_name(path: &String) -> String {
  Path::new(&path)
      .file_stem()
      .and_then(|os_str| os_str.to_str())
      .unwrap_or("")
      .to_string()
}

pub fn extract_file_extension(path: &String) -> String {
    Path::new(&path)
        .extension()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
        .to_string()
}

pub fn get_media_type_by_ext(ext: &String) -> Option<MediaType> {
  return if ACCEPTABLE_VIDEO_FORMATS.iter().any(|e| ext.contains(e)) {
    Some(MediaType::Video)
  } else if ACCEPTABLE_AUDIO_FORMATS.iter().any(|e| ext.contains(e)) {
    Some(MediaType::Audio)
  } else {
    None
  }
}

pub fn get_video_duration(file_path: &Path) -> Result<f64, String> {
  let output = Command::new("ffprobe")
      .args(&[
          "-v", "error",
          "-show_entries", "format=duration",
          "-of", "default=noprint_wrappers=1:nokey=1",
          file_path.to_str().ok_or("Invalid file path")?,
      ])
      .output()
      .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

  if !output.status.success() {
      return Err(format!(
          "ffprobe error: {}",
          String::from_utf8_lossy(&output.stderr)
      ));
  }

  let duration_str = String::from_utf8_lossy(&output.stdout);
  let duration = duration_str
      .trim()
      .parse::<f64>()
      .map_err(|e| format!("Failed to parse duration: {}", e))?;

  Ok(duration)
}

pub fn generate_thumbnail(file_path: &Path, output_path: &Path) -> Result<(), String> {
  let status = Command::new("ffmpeg")
      .args(&[
          "-i", file_path.to_str().ok_or("Invalid file path")?,
          "-ss", "00:00:05",  // Extract at the 5-second mark
          "-vframes", "1",    // Capture a single frame
          "-q:v", "2",        // Quality level (lower is better)
          output_path.to_str().ok_or("Invalid output path")?,
      ])
      .status()
      .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

  if !status.success() {
      return Err("ffmpeg command failed".to_string());
  }

  Ok(())
}
