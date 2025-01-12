use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Clone)]
#[repr(u8)]
pub enum MediaType {
    Video = 0,
    Audio = 1,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMeta {
    pub name: String,
    pub path: String,
    pub duration: u32,
    pub thumbnail_path: Option<String>,
    pub media_type: u8,
    pub created_at: String,
    pub is_local: bool,
    pub size: Option<String>,
}

