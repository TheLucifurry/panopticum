use std::{fmt::Debug, path::PathBuf, time::SystemTime};

use chrono::{DateTime, Utc};
use tauri::{command, AppHandle, Manager, Runtime};
use walkdir::WalkDir;

use crate::{models::FileMeta, utils::fs::{extract_file_extension, extract_file_name, get_media_type_by_ext, path_to_string}};

fn get_all(dir_path: &PathBuf) -> Vec<FileMeta> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|entry| {
            if entry.is_err() {
                return None;
            }

            let dir = entry.unwrap();
            if !dir.file_type().is_file() {
                return None;
            }

            let created_at = <SystemTime as Into<DateTime<Utc>>>::into(dir.metadata().unwrap().created().unwrap().clone()).format("%+").to_string();
            let path = path_to_string(&dir.into_path());
            let name = extract_file_name(&path.to_owned());
            let ext = extract_file_extension(&path.to_owned());

            let media_type = get_media_type_by_ext(&ext);
            if media_type.is_none() {
                return None;
            }

            return Some(FileMeta {
                name,
                path,
                created_at,
                is_local: true,
                media_type: media_type.unwrap() as u8,
            });
        })
        .collect()
}


#[command]
pub fn import_get_all<R: Runtime>(app: AppHandle<R>) -> Result<Vec<FileMeta>, String> {
    let path_module = &app.app_handle().path();
    let file_paths = vec![
        get_all(&path_module.video_dir().expect("Failed to get videos directory")),
        get_all(&path_module.audio_dir().expect("Failed to get audios directory")),
    ].into_iter().flatten().collect();

    Ok(file_paths)
}
