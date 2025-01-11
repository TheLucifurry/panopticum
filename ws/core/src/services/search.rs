use std::path::PathBuf;

use rust_search::SearchBuilder;
use tauri::{self, Manager, Runtime};
use tauri::{command, AppHandle};

use crate::consts::{ConstVecString, ACCEPTABLE_AUDIO_FORMATS, ACCEPTABLE_VIDEO_FORMATS};
use crate::models::{FileMeta, MediaType};
use crate::utils::fs::extract_file_name;

fn search_files(
    path: PathBuf,
    search_input: String,
    exts: ConstVecString,
    media_type: MediaType,
) -> Result<Vec<FileMeta>, String> {
    let dir_path = &path.to_path_buf().to_owned();
    let file_paths: Vec<FileMeta> = exts
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
        .map(|path| FileMeta {
            name: extract_file_name(&path),
            path,
            size: None,
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

#[command]
pub fn search_audio_files<R: Runtime>(app: AppHandle<R>, search_input: String) -> Result<Vec<FileMeta>, String> {
    let dir_path = app.app_handle().path().audio_dir().expect("Failed to get audio directory");

    return search_files(dir_path, search_input, ACCEPTABLE_AUDIO_FORMATS, MediaType::Audio);
}

#[command]
pub fn search_video_files<R: Runtime>(app: AppHandle<R>, search_input: String) -> Result<Vec<FileMeta>, String> {
    let dir_path = app.app_handle().path().video_dir().expect("Failed to get video directory");

    return search_files(dir_path, search_input, ACCEPTABLE_VIDEO_FORMATS, MediaType::Video);
}
