use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

use crate::{
    modules::cache::services::FileCacheService,
    utils::fs::{
        check_file_exists, create_dir_if_not_exist, encode_path_to_filename,
        extract_file_extension, extract_file_media_time_length, extract_file_name,
        generate_thumbnail, get_media_type_by_ext, path_to_string,
    },
};
use chrono::{DateTime, Utc};
use panopticum_schemas::{IContentMedia, MediaType};
use tauri::{AppHandle, Manager, Runtime};
use walkdir::WalkDir;

use rust_search::SearchBuilder;

use crate::consts::ConstVecString;

pub struct ContentService {
    file_cache_thumbnail_service: Arc<FileCacheService>,
}

impl ContentService {
    pub fn new(file_cache_thumbnail_service: Arc<FileCacheService>) -> Self {
        Self {
            file_cache_thumbnail_service,
        }
    }

    pub fn get_all<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        dir_path: &PathBuf,
    ) -> Vec<IContentMedia> {
        let cache_dir_path_buf = app
            .app_handle()
            .path()
            .app_cache_dir()
            .expect("Failed to get app cache directory");
        let mut thumbnail_dir_path_buf = cache_dir_path_buf.clone();
        thumbnail_dir_path_buf.push("t");

        create_dir_if_not_exist(&thumbnail_dir_path_buf);

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
                let ext = extract_file_extension(&path.to_owned());

                let mut thumbnail_dir = thumbnail_dir_path_buf.clone();
                let thumbnail_name = format!(
                    "{}{}",
                    encode_path_to_filename(&file_path.to_str()?),
                    ".png"
                );
                thumbnail_dir.push(thumbnail_name);
                let thumbnail_path = path_to_string(thumbnail_dir.as_path());

                let maybe_media_type = get_media_type_by_ext(&ext);
                if maybe_media_type.is_none() {
                    log::warn!(
                        "Detected unhandled media type by path: {}",
                        &dir.clone().path().display()
                    );
                    return None;
                }
                let media_type = maybe_media_type.unwrap();
                let mut duration: u32 = 0;

                match media_type {
                    MediaType::Video => {
                        let thumbnail_path_string = thumbnail_path.clone();
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
                    thumbnail_path: Some(thumbnail_path),
                    created_at,
                    is_local: true,
                    media_type: media_type as u8,
                    size: Some(size),
                });
            })
            .collect()
    }

    pub fn search_files(
        &self,
        path: PathBuf,
        search_input: String,
        exts: ConstVecString,
        media_type: MediaType,
    ) -> Result<Vec<IContentMedia>, String> {
        let dir_path = &path.to_path_buf().to_owned();
        let file_paths: Vec<IContentMedia> = exts
            .into_iter()
            .map(|ext| -> Vec<String> {
                SearchBuilder::default()
                    .search_input(&search_input)
                    .location(dir_path)
                    .ext(ext.to_owned())
                    .depth(16) // TODO: Research for
                    .hidden()
                    .build()
                    .collect()
            })
            .flat_map(|paths| paths)
            .map(|path| IContentMedia {
                name: extract_file_name(&path),
                path,
                size: None,
                duration: 0,
                thumbnail_path: Some(String::from("")),
                created_at: String::from(""),
                is_local: true,
                media_type: match media_type {
                    MediaType::Video => 0,
                    MediaType::Audio => 1,
                },
            })
            .collect();
        // TODO: filter by unique paths

        Ok(file_paths)
    }
}
