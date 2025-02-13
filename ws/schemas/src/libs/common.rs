use serde::{Deserialize, Serialize};

// Cant be converted by typeshare
#[derive(Serialize, Deserialize, Debug)]
pub struct ValueLabelPair(String, String);
