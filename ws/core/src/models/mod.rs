use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Debug, Serialize, Clone)]
pub enum MediaType {
    Video = 0,
    Audio = 1,
}
// MediaMap
// 001 - video/no-video
// 010 - audio/no-audio
// 100 - short/long

// #[derive(Serialize)]
pub struct FileMeta {
    pub name: String,
    pub path: String,
    pub media_type: u8,
    pub is_local: bool,
    // media_type: MediaType,
    // size: u64,
}

impl Serialize for FileMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut s = serializer.serialize_struct("FileMeta", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("path", &self.path)?;
        s.serialize_field("media_type", &self.media_type)?;
        s.serialize_field("is_local", &self.is_local)?;
        s.end()
    }
}