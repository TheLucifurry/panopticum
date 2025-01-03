use tauri::{ipc, Wry};

pub mod import;
pub mod search;

pub fn invoke_handler(invoke: ipc::Invoke<Wry>) -> bool {
    const HANDLER: fn(ipc::Invoke<Wry>) -> bool = tauri::generate_handler![
        search::search_audio_files,
        search::search_video_files,
        import::import_scan_audio_files,
    ];
    HANDLER(invoke)
}
