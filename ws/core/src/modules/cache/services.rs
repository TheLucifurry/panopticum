use std::path::{Path, PathBuf};

use crate::utils::fs::{check_file_exists, create_dir_if_not_exist, encode_path_to_filename, generate_thumbnail, path_to_string};

pub struct FileCacheService {
    base_path: PathBuf,
}

impl FileCacheService {
    pub fn new(base_path: PathBuf) -> Self {
        println!("FileCacheService.base_path: {}", base_path.display());
        create_dir_if_not_exist(&base_path);
        Self { base_path }
    }

    pub fn get_thumbnail_path(&self, video_file_path: &Path) -> Option<String> {
        let mut thumbnail_dir = self.base_path.clone();
        let thumbnail_name = format!(
            "{}{}",
            encode_path_to_filename(&video_file_path.to_str()?),
            ".png"
        );
        thumbnail_dir.push(thumbnail_name);
        let result = path_to_string(thumbnail_dir.as_path());

        // TODO: Make operation async
        if !check_file_exists(&result) {
            let result = generate_thumbnail(&video_file_path, Path::new(&result));
            if result.is_err() {
                log::error!("{:?}", result.err());
            }
        }

        Some(result)
    }
}
