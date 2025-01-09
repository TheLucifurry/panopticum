use tauri::{command, AppHandle, Manager, Runtime};
use walkdir::WalkDir;

use crate::{models::{FileMeta, MediaType}, utils::fs::{extract_file_name_from_path, path_to_string}};

#[command]
pub fn import_get_all<R: Runtime>(app: AppHandle<R>) -> Result<Vec<FileMeta>, String> {
    let media_type = MediaType::Video;
    let dir_path = &app.app_handle().path().video_dir().expect("Failed to list files in directory");
    let file_paths = WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok) // Ignore entries with errors
        // .filter(|entry| entry.file_type().is_file()) // Only return files
        .map(|dir| path_to_string(&dir.into_path()))
        .map(|path| FileMeta {
            name: extract_file_name_from_path(&path.to_owned()),
            path,
            is_local: true,
            media_type: match media_type {
                MediaType::Video => 0,
                MediaType::Audio => 1,
            },
        })
        .collect();

    Ok(file_paths)
}
