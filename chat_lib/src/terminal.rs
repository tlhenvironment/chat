use std::{process::Command, time::Duration};

use tokio::{io::{self, AsyncBufReadExt as _, BufReader}, sync::{mpsc::{Receiver, Sender}, oneshot}};

use crate::{chat::{ChatAlignment, FullMessage, Message}, state::StateMessage};

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

pub async fn capture_user_input(tx_chat: Sender<FullMessage>, tx_local_chat: Sender<Message>, tx_state: Sender<StateMessage>){
    // Capture user input asynchronously
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    //get username from state
    let (tx, rx) = oneshot::channel();
    tx_state.send(StateMessage::Get(crate::state::StateMessageGet::GetName(tx))).await.unwrap();
    let username = rx.await.unwrap();

    //get random hash from state
    let (tx, rx) = oneshot::channel();
    tx_state.send(StateMessage::Get(crate::state::StateMessageGet::GetRandomKey(tx))).await.unwrap();
    let random_hash = rx.await.unwrap();

    loop {
        // Read the input line asynchronously
        if let Some(line) = lines.next_line().await.unwrap() {
            let message = Message::new(username.to_string(), line, random_hash.to_string());
            let fullmessage = FullMessage::new(message.clone(), crate::chat::ChatAlignment::Right);
            tx_local_chat.send(message).await.unwrap();
            tx_chat.send(fullmessage).await.unwrap();
        }
    }
}

pub async fn startup_message(){
    println!("--------------- Chat application ---------------");
    println!("\n \n");

    tokio::time::sleep(Duration::from_secs(1)).await;
}

pub async fn print_messages_loop(mut rx: Receiver<FullMessage>, tx: Sender<StateMessage>) {
    //get random hash from state
    let(tx_hash, rx_hash) = oneshot::channel();
    tx.send(StateMessage::Get(crate::state::StateMessageGet::GetRandomKey(tx_hash))).await.unwrap();
    let random_hash_state = &rx_hash.await.unwrap();

    loop {
        let fullmessage = rx.recv().await.unwrap();
        let random_hash_message = fullmessage.get_message().get_random_hash();

        if (random_hash_state != random_hash_message) || (fullmessage.get_alingment() == &ChatAlignment::Right) {
            fullmessage.chat_print().unwrap();
        }
        
    };
}

