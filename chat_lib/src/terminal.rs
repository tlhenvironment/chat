use std::{process::Command, time::Duration};

use tokio::{io::{self, AsyncBufReadExt as _, BufReader}, sync::mpsc::{Receiver, Sender}};

use crate::chat::{FullMessage, Message};

#[derive(Debug)]
pub enum TermError {
    TermSizeError,

}

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear the terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear the terminal");
    }
}

pub async fn capture_user_input(tx: Sender<FullMessage>, username: &str){
    // Capture user input asynchronously
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    loop {
        // Read the input line asynchronously
        if let Some(line) = lines.next_line().await.unwrap() {
            let message = Message::new(username.to_string(), line);
            let fullmessage = FullMessage::new(message, crate::chat::ChatAlignment::Right);
            tx.send(fullmessage).await.unwrap();
        }
    }
}

pub async fn startup_message(){
    println!("--------------- Chat application ---------------");
    println!("\n \n");

    tokio::time::sleep(Duration::from_secs(1)).await;
}

pub async fn print_messages(mut rx: Receiver<FullMessage>) {
    loop {
        let fullmessage = rx.recv().await.unwrap();
        fullmessage.chat_print().unwrap();
    }
}