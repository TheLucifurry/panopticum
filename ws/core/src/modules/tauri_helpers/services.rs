
use std::path::{Path, PathBuf};

use tauri::{path::BaseDirectory, AppHandle, Manager};

pub struct TauriHelpersService{ }

impl TauriHelpersService {
    pub fn new() -> Self {
        Self { }
    }

    pub fn base_directory_to_path(&self, app_handle: &AppHandle, base: BaseDirectory) -> Option<PathBuf> {
        let resolver = app_handle.path();
        match base {
            BaseDirectory::Video => resolver.video_dir().ok(),
            BaseDirectory::Audio => resolver.audio_dir().ok(),
            BaseDirectory::Cache => resolver.cache_dir().ok(),
            BaseDirectory::Config => resolver.config_dir().ok(),
            BaseDirectory::Data => resolver.data_dir().ok(),
            BaseDirectory::Desktop => resolver.desktop_dir().ok(),
            BaseDirectory::Document => resolver.document_dir().ok(),
            BaseDirectory::Download => resolver.download_dir().ok(),
            BaseDirectory::LocalData => resolver.local_data_dir().ok(),
            BaseDirectory::Picture => resolver.picture_dir().ok(),
            BaseDirectory::Public => resolver.public_dir().ok(),
            BaseDirectory::Runtime => resolver.runtime_dir().ok(),
            BaseDirectory::Template => resolver.template_dir().ok(),
            // Добавьте обработку других вариантов, если потребуется
            _ => None,
        }
    }

    pub fn path_to_base_directory(&self, app_handle: &AppHandle, path: &Path) -> Option<BaseDirectory> {
        // Список всех известных директорий
        let directories = [
            BaseDirectory::Video,
            BaseDirectory::Audio,
            BaseDirectory::Cache,
            BaseDirectory::Config,
            BaseDirectory::Data,
            BaseDirectory::Desktop,
            BaseDirectory::Document,
            BaseDirectory::Download,
            BaseDirectory::LocalData,
            BaseDirectory::Picture,
            BaseDirectory::Public,
            BaseDirectory::Runtime,
            BaseDirectory::Template,
        ];

        for &base in directories.iter() {
            if let Some(base_path) = self.base_directory_to_path(app_handle, base) {
                if path.starts_with(&base_path) {
                    return Some(base);
                }
            }
        }
        None
    }
}
