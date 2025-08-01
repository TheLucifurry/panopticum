use blake3;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use panopticum_schemas::MediaType;

use crate::consts::{ACCEPTABLE_AUDIO_FORMATS, ACCEPTABLE_VIDEO_FORMATS};

pub fn create_dir_if_not_exist(path: &PathBuf) {
    fs::create_dir_all(path)
        .expect(&format!("Failed to create dir {}", &path.display()).to_string());
}

pub fn encode_path_to_filename(file_path: &str) -> String {
    blake3::hash(file_path.as_bytes()).to_hex().to_string()
}

pub fn check_file_exists(file_path: &String) -> bool {
    Path::new(&file_path).exists()
}

pub fn change_file_name_in_path(original_path: &Path, new_file_name: &str) -> Option<String> {
    let path = Path::new(original_path);
    let parent = path.parent()?;
    let new_path = parent.join(new_file_name);
    new_path.to_str().map(|s| s.to_string())
}

pub fn path_buf_join(path_buf: PathBuf, path_str: &str) -> PathBuf {
    let mut mut_path_buf = path_buf.clone();
    mut_path_buf.push(path_str);
    mut_path_buf
}

pub fn path_to_string(path: &Path) -> String {
    path.to_str()
        .expect("Failed to convert String to Path")
        .to_string()
}

pub fn extract_file_media_time_length(file_path: &String) -> Result<u32, String> {
    let output = Command::new("ffprobe")
        .args([
            "-i",
            &file_path,
            "-show_entries",
            "format=duration",
            "-v",
            "quiet",
            "-of",
            "csv=p=0",
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "ffprobe error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let duration = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("Failed to parse duration: {}", e))?;

    Ok(duration as u32)
}

pub fn extract_file_name(path: &String) -> String {
    Path::new(&path)
        .file_stem()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or_default()
        .to_string()
}

pub fn extract_file_extension(path: &String) -> String {
    Path::new(&path)
        .extension()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or_default()
        .to_string()
}

pub fn get_media_type_by_ext(ext: &String) -> Option<MediaType> {
    return if ACCEPTABLE_VIDEO_FORMATS.iter().any(|e| ext.contains(e)) {
        Some(MediaType::Video)
    } else if ACCEPTABLE_AUDIO_FORMATS.iter().any(|e| ext.contains(e)) {
        Some(MediaType::Audio)
    } else {
        None
    };
}

pub fn get_video_duration(file_path: &Path) -> Result<f64, String> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
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
