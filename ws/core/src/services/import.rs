use rust_search::SearchBuilder;
use tauri::{self, Manager, Runtime};
use tauri::{command, AppHandle};

use crate::consts::ACCEPTABLE_AUDIO_FORMATS;

#[command]
pub fn import_scan_audio_files<R: Runtime>(app: AppHandle<R>) -> Result<Vec<String>, String> {
    // let file_dialog_result = app.dialog().file().blocking_pick_folder();
    // if file_dialog_result.is_none() {
    //     return Err("Directory wasn't chosen".into());
    // }
    // let dir_path = file_dialog_result.unwrap();
    let dir_path = &app
        .app_handle()
        .path()
        .video_dir()
        .unwrap()
        .to_path_buf()
        .to_owned();
    dbg!(&dir_path);

    let file_paths: Vec<String> = ACCEPTABLE_AUDIO_FORMATS
        .into_iter()
        .map(|ext| -> Vec<String> {
            SearchBuilder::default()
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
