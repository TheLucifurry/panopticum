mod services;

pub mod consts;
pub mod models;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(services::invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
