use tauri::{ipc, App, Wry};
use std::sync::Arc;
use content::{commands, services::ContentService};

mod content;

#[derive(Clone)]
pub struct Modules {
    pub content_service: Arc<ContentService>,
}

pub type M<'a> = State<'a, Modules>;

impl Modules {
    pub fn new(app: &mut App) -> Self {
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
