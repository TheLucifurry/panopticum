use serde::{Deserialize, Serialize};
use typeshare::typeshare;

// Cant be converted by typeshare
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ValueLabelPair(pub String, pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PathNode {
    String(String),
    Pair(ValueLabelPair),
}

impl PathNode {
    pub fn as_value(&self) -> &str {
        match self {
            PathNode::String(s) => s.as_str(),
            PathNode::Pair(ValueLabelPair(value, _)) => value.as_str(),
        }
    }
    pub fn as_label(&self) -> &str {
        match self {
            PathNode::String(s) => s.as_str(),
            PathNode::Pair(ValueLabelPair(_, label)) => label.as_str(),
        }
    }
}

pub type PathNodes = Vec<PathNode>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paginated {
    #[typeshare(serialized_as = "number")]
    pub current: usize, // Whether there is a next page
    #[typeshare(serialized_as = "number")]
    pub size: usize, // Items per page
    #[typeshare(serialized_as = "number")]
    pub total: usize, // Total number of items
}

impl Paginated {
    pub fn new() -> Self {
        Self {
            current: 0,
            size: 0,
            total: 0,
        }
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn size(mut self, value: usize) -> Self {
        self.size = value;
        self
    }

    pub fn total(mut self, value: usize) -> Self {
        self.total = value;
        self
    }
}

#[typeshare]
#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum MediaType {
    Video = 0,
    Audio = 1,
}
