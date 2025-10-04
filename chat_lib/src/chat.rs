use std::{ffi::os_str::Display, io::{self, Write as _}};

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::{chat, terminal::TermError};

pub enum ChatAlignment {
    Right,
    Left,
}

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

pub struct FullMessage {
    message: Message,
    alignment: ChatAlignment,
}

impl FullMessage {
    pub fn new(messsage: Message, alignment: ChatAlignment) -> Self {
        Self {
            message: messsage,
            alignment,
        }
    }

  pub fn chat_print(&self) -> Result<(), TermError> {
        let (width, _) = match term_size::dimensions() {
            Some((w,h)) => (w,h),
            None => {
                return Err(chat::TermError::TermSizeError);
            },
        };

        match self.alignment {
            ChatAlignment::Left => {
                println!("{:<width$}:", self.message.sender, width = width - 1);
                println!("\t{:<width$}", self.message.text, width = width);
            },
            ChatAlignment::Right => {
                println!("{:>width$}:", self.message.sender, width = width - 1);
                println!("{:>width$}", self.message.text, width = width - 4);
            },
        }
        io::stdout().flush().unwrap(); // Ensure the message is printed immediately

        Ok(())
    }

}

// Implementing the Display trait for Message
impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}: \n\t{}", self.sender, self.text)
    }
}

pub async fn write_loop() {

}