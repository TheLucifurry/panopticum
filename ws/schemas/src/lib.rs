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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IContentMedia {
    pub name: String,
    pub path: String,
    pub duration: u32,
    pub thumbnail_path: Option<String>,
    pub media_type: u8,
    pub created_at: String,
    pub is_local: bool,
    pub size: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IContentProvider {
    pub key: String,
    pub name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    Media,
    List,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IContentList {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub items: Vec<IContentNode>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "body", rename_all = "camelCase")]
pub enum IContentNode {
    Media(IContentMedia),
    List(IContentList),
}
