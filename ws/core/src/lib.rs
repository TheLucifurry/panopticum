use modules::Modules;
use tauri::Manager;

pub mod prelude;
pub mod consts;
pub mod modules;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_devtools::init());
    }

    builder
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
