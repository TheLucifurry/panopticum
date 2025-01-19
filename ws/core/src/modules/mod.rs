use tauri::{ipc, Wry};
use std::sync::Arc;
use content::{commands, services::ContentService};

mod content;

#[derive(Clone)]
pub struct AppState {
    pub content_service: Arc<ContentService>,
}

impl AppState {
    pub fn new() -> Self {
        let content_service = Arc::new(ContentService::new());

        Self {
            content_service,
        }
    }
}

pub fn invoke_handler(invoke: ipc::Invoke<Wry>) -> bool {
    const HANDLER: fn(ipc::Invoke<Wry>) -> bool = tauri::generate_handler![
        commands::content_get_all,
        commands::search_audio_files,
        commands::search_video_files,
    ];
    HANDLER(invoke)
}
