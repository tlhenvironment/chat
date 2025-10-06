use chat_lib::chat::FullMessage;
use chat_lib::state::{state_loop, State, StateMessage};
use config::Config;
use chat_lib::{chat::Message};
use chat_lib::terminal::{capture_user_input, clear_terminal, print_messages_loop, startup_message};
use env_logger::Builder;
use log::info;
use tokio::io::{self, AsyncBufReadExt as _, BufReader};
use std::fmt::Alignment;
use std::sync::mpsc::{Receiver, Sender};
use std::{env, thread, time::{self, Duration}};
use tokio::sync::mpsc;

use crate::net::{mqtt_connect};

mod net;

#[tokio::main]
async fn main() {
    clear_terminal();        

    // Initialize a logger with the default configuration
    Builder::new()
        .filter_level(log::LevelFilter::Debug) // Set desired log level
        .init();

    // Log messages
    log::debug!("This is a debug message.");
    log::info!("This is an info message.");
    log::warn!("This is a warning message.");
    log::error!("This is an error message.");
    
    //read in settings
    let settings = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .build()
        .unwrap();

    let username = settings.get_string("username").unwrap();

    //state
    let state = State::new(username);
    let (tx_state, rx_state) = mpsc::channel::<StateMessage>(16);
    let tx_state_print = tx_state.clone();
    let random_hash_state = state.get_random_hash().clone();

    tokio::spawn(async move {
        state_loop(state, rx_state).await
    });

    //test
    //give output
    startup_message().await;

    //make channel to communicate between the task that prints
    //and the tasks that receive messages
    let (tx_chat, rx_chat) = mpsc::channel::<FullMessage>(16);
    
    let tx_chat_mqtt = tx_chat.clone();
    let tx_chat_input = tx_chat.clone();
 
    //make channel to communicate between local chat input and mqtt sending
    let (tx_local_chat, rx_local_chat) = mpsc::channel::<Message>(16);

    // //wait a bit to clear
    tokio::time::sleep(Duration::from_secs(1)).await;
    clear_terminal();
    tokio::time::sleep(Duration::from_secs(1)).await;

    //task to print messages
    tokio::spawn(async move {
        print_messages_loop(rx_chat, tx_state_print).await
    });

    //task that handles mqtt connection
    tokio::spawn(async move{
        mqtt_connect(settings, tx_chat_mqtt, rx_local_chat, random_hash_state).await
    });

    //task that handles local input
    let handle = tokio::spawn(async move {
        capture_user_input(tx_chat_input, tx_local_chat, tx_state).await;
    });
    handle.await.unwrap();


}
