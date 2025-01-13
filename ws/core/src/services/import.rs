use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::utils::fs::{
    change_file_name_in_path, check_file_exists, extract_file_extension,
    extract_file_media_time_length, extract_file_name, generate_thumbnail, get_media_type_by_ext,
    path_to_string,
};
use chrono::{DateTime, Utc};
use panopticum_schemas::{IContentMedia, MediaType};
use tauri::{command, AppHandle, Manager, Runtime};
use walkdir::WalkDir;

fn get_all(dir_path: &PathBuf) -> Vec<IContentMedia> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|entry| {
            if entry.is_err() {
                log::warn!(
                    "Failed to get file by path: {}",
                    entry.unwrap().path().display()
                );
                return None;
            }

            let dir = entry.unwrap();
            if !dir.file_type().is_file() {
                return None;
            }

            let file_path = &dir.path();
            let path = path_to_string(file_path);
            let metadata = dir.metadata().unwrap();
            let created_at =
                <SystemTime as Into<DateTime<Utc>>>::into(metadata.created().unwrap().clone())
                    .format("%+")
                    .to_string();
            let size = metadata.len().to_string();
            let name = extract_file_name(&path.to_owned());
            let thumbnail_path =
                change_file_name_in_path(file_path, &format!("{}{}", name, "_thumbnail.png"));
            let ext = extract_file_extension(&path.to_owned());

            let maybe_media_type = get_media_type_by_ext(&ext);
            if maybe_media_type.is_none() {
                // log::warn!("Detected unhandled media type by path: {}", &dir.clone().path().display());
                return None;
            }
            let media_type = maybe_media_type.unwrap();
            let mut duration: u32 = 0;

            match media_type {
                MediaType::Video => {
                    let thumbnail_path_string = thumbnail_path.clone().unwrap();
                    let maybe_duration = extract_file_media_time_length(&path);
                    if maybe_duration.is_ok() {
                        duration = maybe_duration.unwrap();
                    }
                    if !check_file_exists(&thumbnail_path_string) {
                        let result =
                            generate_thumbnail(&dir.path(), Path::new(&thumbnail_path_string));
                        if result.is_err() {
                            log::error!("{:?}", result.err());
                        }
                    }
                }
                _ => {}
            }

            return Some(IContentMedia {
                name,
                path,
                duration,
                thumbnail_path,
                created_at,
                is_local: true,
                media_type: media_type as u8,
                size: Some(size),
            });
        })
        .collect()
}

#[command]
pub fn import_get_all<R: Runtime>(app: AppHandle<R>) -> Result<Vec<IContentMedia>, String> {
    let path_module = &app.app_handle().path();
    let file_paths = vec![
        get_all(
            &path_module
                .video_dir()
                .expect("Failed to get videos directory"),
        ),
        get_all(
            &path_module
                .audio_dir()
                .expect("Failed to get audios directory"),
        ),
    ]
    .into_iter()
    .flatten()
    .collect();

    Ok(file_paths)
}
