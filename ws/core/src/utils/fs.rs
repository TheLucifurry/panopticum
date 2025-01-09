use std::path::Path;

pub fn path_to_string(path: &Path) -> String {
  path.to_str() // Convert the Path to an Option<&str>
      .unwrap_or("Failed to convert String to Path") // Fallback to an empty string if conversion fails
      .to_string() // Convert &str to String
}

pub fn extract_file_name_from_path(path: &String) -> String {
  Path::new(&path)
      .file_stem() // Extract the file name
      .and_then(|os_str| os_str.to_str()) // Convert it to a string slice
      .unwrap_or("") // Fallback to an empty string if not found
      .to_string() // Convert to String
}