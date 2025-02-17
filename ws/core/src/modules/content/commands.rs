use crate::{
    consts::{ACCEPTABLE_AUDIO_FORMATS, ACCEPTABLE_VIDEO_FORMATS},
    modules::M,
    s,
    utils::schemas::path_nodes_to_path_buf,
};
use panopticum_schemas::{ContentNode, IContentMedia, MediaType, PathNodes};
use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub fn content_get_dir_node<R: Runtime>(
    modules: M,
    app: AppHandle<R>,
    location: Option<PathNodes>,
) -> Result<ContentNode, String> {
    let service = modules.content_service.clone();
    let path_module = &app.app_handle().path();

    let file_paths = match location {
        Some(mut loc) => {
            let first_node = loc.remove(0);
            let path = path_nodes_to_path_buf(&loc);
            let base_path = match first_node.as_value() {
                "Video" => &path_module
                    .video_dir()
                    .expect("Failed to get videos directory"),
                &_ => return Err(s!("Path not handled")),
            };
            let full_path = base_path.join(&path);
            log::debug!("full_path: {:?}", full_path.display());

            service.get_dir_node_root(&full_path)?
        }
        None => ContentNode::from_items(
            vec![
                service.get_dir_node(
                    &path_module
                        .video_dir()
                        .expect("Failed to get videos directory"),
                )?,
                service.get_dir_node(
                    &path_module
                        .audio_dir()
                        .expect("Failed to get audios directory"),
                )?,
            ],
            None,
            None,
        ),
    };

    Ok(file_paths)
}

#[tauri::command]
pub fn search_audio_files<R: Runtime>(
    modules: M,
    app: AppHandle<R>,
    search_input: String,
) -> Result<Vec<IContentMedia>, String> {
    let service = modules.content_service.clone();
    let dir_path = app
        .app_handle()
        .path()
        .audio_dir()
        .expect("Failed to get audio directory");

    return service.search_files(
        dir_path,
        search_input,
        ACCEPTABLE_AUDIO_FORMATS,
        MediaType::Audio,
    );
}

#[tauri::command]
pub fn search_video_files<R: Runtime>(
    modules: M,
    app: AppHandle<R>,
    search_input: String,
) -> Result<Vec<IContentMedia>, String> {
    let service = modules.content_service.clone();
    let dir_path = app
        .app_handle()
        .path()
        .video_dir()
        .expect("Failed to get video directory");

    return service.search_files(
        dir_path,
        search_input,
        ACCEPTABLE_VIDEO_FORMATS,
        MediaType::Video,
    );
}
