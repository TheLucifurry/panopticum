use cache::services::FileCacheService;
use content::{commands, services::ContentService};
use ffmpeg::services::FfmpegService;
use std::sync::Arc;
use tauri::{ipc, App, Manager, State, Wry};

use crate::utils::fs::path_buf_join;

mod cache;
mod content;
mod ffmpeg;

#[derive(Clone)]
pub struct Modules {
    pub content_service: Arc<ContentService>,
    pub ffmpeg_service: Arc<FfmpegService>,
}

pub type M<'a> = State<'a, Modules>;

impl Modules {
    pub fn new(app: &mut App) -> Self {
        let path_resolver = app.path();
        let cache_dir_path_buf = path_resolver
            .app_cache_dir()
            .expect("Failed to get app cache directory");
        let thumbnail_dir_path_buf = path_buf_join(cache_dir_path_buf, "t");

        let ffmpeg_service = Arc::new(FfmpegService::new());
        let file_cache_thumbnails_service = Arc::new(FileCacheService::new(ffmpeg_service.clone(), thumbnail_dir_path_buf));
        let content_service = Arc::new(ContentService::new(file_cache_thumbnails_service.clone()));

        Self {
            content_service,
            ffmpeg_service,
        }
    }
}

pub fn invoke_handler(invoke: ipc::Invoke<Wry>) -> bool {
    const HANDLER: fn(ipc::Invoke<Wry>) -> bool = tauri::generate_handler![
        commands::content_get_dir_node,
        commands::search_audio_files,
        commands::search_video_files,
    ];
    HANDLER(invoke)
}
