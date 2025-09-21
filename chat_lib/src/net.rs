use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Decode, Encode)]
pub struct Message {
    sender: String,
    text: String,
}