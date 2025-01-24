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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContentProviderViewKey {
    Main,
    Search,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IContentProviderViews {
    pub main: String,
    pub search: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IContentProvider {
    pub key: String,
    pub name: String,
    pub views: IContentProviderViews,
}

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
    pub items: Vec<ContentNode>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "body", rename_all = "camelCase")]
pub enum ContentNode {
    Media(IContentMedia),
    List(IContentList),
}
