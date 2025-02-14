use serde::{Deserialize, Serialize};
use typeshare::typeshare;

// Cant be converted by typeshare
#[derive(Serialize, Deserialize, Debug)]
pub struct ValueLabelPair(String, String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PathNode {
    // Variant for a single string
    String(String),
    // Variant for a pair of strings
    Pair(ValueLabelPair),
}

pub type PathNodes = Vec<PathNode>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IPaginated {
    #[typeshare(serialized_as = "number")]
    pub current: usize, // Whether there is a next page
    #[typeshare(serialized_as = "number")]
    pub size: usize, // Items per page
    #[typeshare(serialized_as = "number")]
    pub total: usize, // Total number of items
}

#[typeshare]
#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum MediaType {
    Video = 0,
    Audio = 1,
}
