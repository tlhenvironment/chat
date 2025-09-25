use std::ffi::os_str::Display;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Decode, Encode)]
pub struct Message {
    sender: String,
    text: String,
}

impl Message {
    pub fn new(sender: String, text: String) -> Self {
        Message {
            sender,
            text,
        }
    }
}

// Implementing the Display trait for Message
impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}: {}", self.sender, self.text)
    }
}
