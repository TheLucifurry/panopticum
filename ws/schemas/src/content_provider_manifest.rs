use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

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
pub struct IContentProviderV0 {
    pub name: String,
    pub version: String,
    pub icons: HashMap<String, String>,
    pub description: Option<String>,
    pub views: IContentProviderViews,

    // TODO: Implement on demand.
    // pub locales: HashMap<String, HashMap<String, String>>,
    // pub permissions: HashMap<String, { required: bool }>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IContentProviderManifestWrapper<T> {
    pub id: String,
    pub version: u8,
    pub data: T,
}

#[typeshare]
pub type IContentProviderManifestV0 = IContentProviderManifestWrapper<IContentProviderV0>;

#[typeshare]
pub type IContentProviderManifest = IContentProviderManifestV0; // Latest
