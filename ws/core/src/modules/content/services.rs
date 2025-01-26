use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

use crate::{
    consts::get_all_acceptable_file_formats,
    modules::cache::services::FileCacheService,
    utils::{fs::{
        extract_file_extension, extract_file_media_time_length, extract_file_name,
        get_media_type_by_ext, path_to_string,
    }, std_helpers::push_if_some},
};
use chrono::{DateTime, Utc};
use panopticum_schemas::{ContentNode, ContentNodeType, IContentList, IContentMedia, IContentPreview, IPaginated, MediaType};
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

    fn get_dir_files_previews(&self, entry: &DirEntry) -> Vec<ContentNode> {
        let mut items: Vec<ContentNode> = Vec::new();

        let entries = WalkDir::new(entry.path())
            .max_depth(1)
            .into_iter()
            .filter_map(|entry| entry.ok());

        for (i, entry) in entries.enumerate() {
            items.push(ContentNode::Preview(IContentPreview {
                r#type: ContentNodeType::Media,
                pict: self.file_cache_thumbnail_service.get_thumbnail_path(entry.path()),
            }));
            if i == 2 {
                break;
            }
        }

        items
    }

    fn get_dir_content_node(&self, entry: &DirEntry) -> Option<ContentNode> {
        let media_count = self.count_files_in_dir(&entry.path());

        if media_count == 0 {
            return None;
        }

        Some(ContentNode::List(IContentList {
            name: get_entry_name(&entry),
            page: IPaginated {
                current: 0,
                size: 3,
                total: media_count,
            },
            items: self.get_dir_files_previews(&entry),
        }))
    }

    pub fn get_dir_node(&self, dir_path: &PathBuf) -> ContentNode {
        let count: usize = 5;
        let entries = WalkDir::new(dir_path)
            .max_depth(1) // Only process one level
            .into_iter()
            .filter_map(|entry| entry.ok()); // Filter out errors
        let media_count = self.count_files_in_dir(&dir_path.as_path());

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
            page: IPaginated {
                current: 0,
                size: items.len(),
                total: media_count,
            },
            items,
        })
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
