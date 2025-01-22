use crate::{consts::{ACCEPTABLE_AUDIO_FORMATS, ACCEPTABLE_VIDEO_FORMATS}, modules::M};
use panopticum_schemas::{ContentNode, IContentMedia, MediaType};
use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub fn content_get_all<R: Runtime>(modules: M, app: AppHandle<R>) -> Result<Vec<ContentNode>, String> {
    let service = modules.content_service.clone();
    let path_module = &app.app_handle().path();
    let file_paths = vec![
        service.get_all(
            &path_module
                .video_dir()
                .expect("Failed to get videos directory"),
        ),
        service.get_all(
            &path_module
                .audio_dir()
                .expect("Failed to get audios directory"),
        ),
    ]
    .into_iter()
    .flatten()
    .collect();

    Ok(file_paths)
}

#[tauri::command]
pub fn search_audio_files<R: Runtime>(modules: M, app: AppHandle<R>, search_input: String) -> Result<Vec<IContentMedia>, String> {
    let service = modules.content_service.clone();
    let dir_path = app.app_handle().path().audio_dir().expect("Failed to get audio directory");

    return service.search_files(dir_path, search_input, ACCEPTABLE_AUDIO_FORMATS, MediaType::Audio);
}

#[tauri::command]
pub fn search_video_files<R: Runtime>(modules: M, app: AppHandle<R>, search_input: String) -> Result<Vec<IContentMedia>, String> {
    let service = modules.content_service.clone();
    let dir_path = app.app_handle().path().video_dir().expect("Failed to get video directory");

    return service.search_files(dir_path, search_input, ACCEPTABLE_VIDEO_FORMATS, MediaType::Video);
}
