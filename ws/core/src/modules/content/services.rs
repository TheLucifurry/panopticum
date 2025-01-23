use std::{path::PathBuf, sync::Arc, time::SystemTime};

use crate::{
    modules::cache::services::FileCacheService,
    utils::fs::{
        extract_file_extension, extract_file_media_time_length, extract_file_name,
        get_media_type_by_ext, path_to_string,
    },
};
use chrono::{DateTime, Utc};
use panopticum_schemas::{ContentNode, IContentList, IContentMedia, MediaType};
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

    pub fn get_all(&self, dir_path: &PathBuf) -> Vec<ContentNode> {
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
                    if !dir.file_type().is_dir() {
                        return None;
                    }
                    return Some(ContentNode::List(IContentList {
                        title: String::from("asd"),
                        description: None,
                        id: String::from("vxc"),
                        items: vec![],
                    }));
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

                let mut thumbnail_path = String::new();

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
                        thumbnail_path = self
                            .file_cache_thumbnail_service
                            .get_thumbnail_path(&file_path)?;
                        let maybe_duration = extract_file_media_time_length(&path);
                        if maybe_duration.is_ok() {
                            duration = maybe_duration.unwrap();
                        }
                    }
                    _ => {}
                }

                return Some(ContentNode::Media(IContentMedia {
                    name,
                    path,
                    duration,
                    thumbnail_path: Some(thumbnail_path),
                    created_at,
                    is_local: true,
                    media_type: media_type as u8,
                    size: Some(size),
                }));
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
