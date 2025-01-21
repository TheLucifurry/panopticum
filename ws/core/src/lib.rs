use modules::Modules;
use tauri::Manager;

pub mod consts;
pub mod utils;
pub mod modules;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let modules = Modules::new(app);
            app.manage(modules);
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(modules::invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
