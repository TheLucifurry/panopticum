use rust_search::SearchBuilder;
use tauri::{self, Manager, Runtime};
use tauri::{command, AppHandle};

use crate::consts::{ACCEPTABLE_AUDIO_FORMATS, ACCEPTABLE_VIDEO_FORMATS};

#[command]
pub fn search_audio_files<R: Runtime>(
    app: AppHandle<R>,
    search_input: String,
) -> Result<Vec<String>, String> {
    let dir_path = &app
        .app_handle()
        .path()
        .audio_dir()
        .unwrap()
        .to_path_buf()
        .to_owned();

    let file_paths: Vec<String> = ACCEPTABLE_AUDIO_FORMATS
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
        .collect();
    // TODO: filter by unique paths

    Ok(file_paths)
}

#[command]
pub fn search_video_files<R: Runtime>(
    app: AppHandle<R>,
    search_input: String,
) -> Result<Vec<String>, String> {
    let dir_path = &app
        .app_handle()
        .path()
        .video_dir()
        .unwrap()
        .to_path_buf()
        .to_owned();

    let file_paths: Vec<String> = ACCEPTABLE_VIDEO_FORMATS
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
        .collect();
    // TODO: filter by unique paths

    Ok(file_paths)
}
