use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

use crate::{
    consts::get_all_acceptable_file_formats,
    modules::cache::services::FileCacheService,
    utils::fs::{
        extract_file_extension, extract_file_media_time_length, extract_file_name,
        get_media_type_by_ext, path_to_string,
    },
};
use chrono::{DateTime, Utc};
use panopticum_schemas::{ContentNode, IContentList, IContentMedia, MediaType};
use walkdir::{DirEntry, WalkDir};

use rust_search::SearchBuilder;

use crate::consts::ConstVecString;

pub struct ContentService {
    acceptable_exts: Vec<&'static str>,
    file_cache_thumbnail_service: Arc<FileCacheService>,
}

pub fn get_entry_name(entry: &DirEntry) -> String {
    entry.file_name().to_string_lossy().to_string()
}

pub fn get_file_name(entry: &DirEntry) -> String {
    extract_file_name(&get_entry_name(&entry))
}

pub fn push_if_some<T>(vec: &mut Vec<T>, value: Option<T>) {
    if let Some(v) = value {
        vec.push(v);
    }
}

impl ContentService {
    pub fn new(file_cache_thumbnail_service: Arc<FileCacheService>) -> Self {
        Self {
            acceptable_exts: get_all_acceptable_file_formats(),
            file_cache_thumbnail_service,
        }
    }

    fn count_files_in_dir<P: AsRef<Path>>(&self, dir: P) -> usize {
        if let Ok(entries) = fs::read_dir(dir) {
            entries
                .filter_map(|entry| entry.ok()) // Filter out invalid entries
                .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)) // Ensure it's a file
                .filter(|entry| {
                    if let Some(ext) = entry.path().extension().and_then(|ext| ext.to_str()) {
                        return self.acceptable_exts.contains(&ext);
                    }
                    false
                })
                .count()
        } else {
            0
        }
    }

    fn get_file_content_node(&self, entry: &DirEntry) -> Option<ContentNode> {
        let name = get_file_name(&entry);
        let file_path = entry.path();
        let path = path_to_string(file_path);
        let metadata = entry.metadata().unwrap();
        let created_at =
            <SystemTime as Into<DateTime<Utc>>>::into(metadata.created().unwrap().clone())
                .format("%+")
                .to_string();
        let size = metadata.len().to_string();
        let ext = extract_file_extension(&path.to_owned());

        let maybe_media_type = get_media_type_by_ext(&ext);
        if maybe_media_type.is_none() {
            log::warn!(
                "Detected unhandled media type by path: {}",
                &entry.clone().path().display()
            );
            return None;
        }
        let media_type = maybe_media_type.unwrap();
        let mut duration: Option<u32> = None;
        let mut thumbnail_path: Option<String> = None;

        match media_type {
            MediaType::Video => {
                thumbnail_path = Some(
                    self.file_cache_thumbnail_service
                        .get_thumbnail_path(&file_path)?,
                );
                let maybe_duration = extract_file_media_time_length(&path);
                if maybe_duration.is_ok() {
                    duration = Some(maybe_duration.unwrap());
                }
            }
            _ => {}
        }

        Some(ContentNode::Media(IContentMedia {
            name,
            path,
            duration,
            thumbnail_path,
            created_at,
            is_local: true,
            media_type: media_type as u8,
            size: Some(size),
        }))
    }

    fn get_dir_content_node(&self, entry: &DirEntry) -> Option<ContentNode> {
        let media_count = self.count_files_in_dir(&entry.path());

        if media_count == 0 {
            return None;
        }

        Some(ContentNode::List(IContentList {
            name: get_entry_name(&entry),
            items: vec![],
        }))
    }

    pub fn get_dir_node(&self, dir_path: &PathBuf) -> ContentNode {
        let count: usize = 5;
        let entries = WalkDir::new(dir_path)
            .max_depth(1) // Only process one level
            .into_iter()
            .filter_map(|entry| entry.ok()); // Filter out errors

        let mut items: Vec<ContentNode> = Vec::new();
        let mut target_dir: Option<DirEntry> = None;

        for (i, entry) in entries.enumerate() {
            if i > 0 && i <= count {
                if entry.file_type().is_file() {
                    push_if_some(&mut items, self.get_file_content_node(&entry));
                } else if entry.file_type().is_dir() {
                    push_if_some(&mut items, self.get_dir_content_node(&entry));
                }
            } else if i == 0 {
                target_dir = Some(entry);
            } else {
                break;
            }
        }

        ContentNode::List(IContentList {
            name: get_entry_name(&target_dir.expect("Failed to read target directory")),
            items,
        })
    }

    pub fn get_all(&self, dir_path: &PathBuf) -> Vec<ContentNode> {
        WalkDir::new(dir_path)
            .max_depth(0)
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
                        name: dir.file_name().to_string_lossy().to_string(),
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
                    duration: Some(duration),
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
                duration: Some(0),
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
