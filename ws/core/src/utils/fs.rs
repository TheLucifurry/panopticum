use std::path::Path;

use crate::{consts::{ACCEPTABLE_AUDIO_FORMATS, ACCEPTABLE_VIDEO_FORMATS}, models::MediaType};

pub fn path_to_string(path: &Path) -> String {
  path.to_str() // Convert the Path to an Option<&str>
      .unwrap_or("Failed to convert String to Path") // Fallback to an empty string if conversion fails
      .to_string() // Convert &str to String
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