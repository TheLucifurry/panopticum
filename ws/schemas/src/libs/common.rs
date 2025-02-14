use serde::{Deserialize, Serialize};
use typeshare::typeshare;

// Cant be converted by typeshare
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ValueLabelPair(pub String, pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PathNode {
    // Variant for a single string
    String(String),
    // Variant for a pair of strings
    Pair(ValueLabelPair),
}

impl PathNode {
    pub fn as_str(&self) -> &str {
        match self {
            PathNode::String(s) => s.as_str(),
            PathNode::Pair(ValueLabelPair(first, _)) => first.as_str(),
        }
    }
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
