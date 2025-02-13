use serde::{Deserialize, Serialize};

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
