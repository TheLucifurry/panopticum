use std::path::PathBuf;

pub struct FileCacheService {
    base_path: PathBuf,
}

impl FileCacheService {
    pub fn new(base_path: PathBuf) -> Self {
        println!("FileCacheService.base_path: {}", base_path.display());
        Self { base_path }
    }
}
