use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
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
#[derive(TypedBuilder)]
pub struct Paginated {
    #[typeshare(serialized_as = "number")]
    #[builder(default=0)]
    pub current: usize, // Whether there is a next page

    #[typeshare(serialized_as = "number")]
    #[builder(default=0)]
    pub size: usize, // Items per page

    #[typeshare(serialized_as = "number")]
    #[builder(default=0)]
    pub total: usize, // Total number of items
}

#[typeshare]
#[derive(Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum MediaType {
    Video = 0,
    Audio = 1,
}
