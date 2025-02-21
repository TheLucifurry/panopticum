use std::{path::{Path, PathBuf}, sync::Arc};

use crate::{ modules::ffmpeg::services::FfmpegService, utils::fs::{check_file_exists, create_dir_if_not_exist, encode_path_to_filename, path_to_string}};

pub struct FileCacheService {
    ffmpeg_service: Arc<FfmpegService>,
    base_path: PathBuf,
}

impl FileCacheService {
    pub fn new(ffmpeg_service: Arc<FfmpegService>, base_path: PathBuf) -> Self {
        log::info!("FileCacheService.base_path: {}", base_path.display());
        create_dir_if_not_exist(&base_path);
        Self {
            ffmpeg_service,
            base_path
        }
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

        if !check_file_exists(&result) {
            let _ = self.ffmpeg_service.generate_thumbnail(&video_file_path, Path::new(&result));
        }

        Some(result)
    }
}
