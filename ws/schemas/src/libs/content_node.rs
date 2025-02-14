use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::common::IPaginated;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IContentMedia {
    pub name: String,
    pub path: String,
    pub duration: Option<u32>,
    pub thumbnail_path: Option<String>,
    pub media_type: u8,
    pub created_at: String,
    pub is_local: bool,
    pub size: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IContentList {
    pub name: String,
    pub page: IPaginated,
    pub items: Vec<ContentNode>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IContentPreview {
    pub r#type: ContentNodeType,
    pub pict: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContentNodeType {
    Media,
    List,
    Preview,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "body", rename_all = "camelCase")]
pub enum ContentNode {
    Media(IContentMedia),
    List(IContentList),
    Preview(IContentPreview),
}
