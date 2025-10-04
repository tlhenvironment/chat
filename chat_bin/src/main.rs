use chat_lib::chat::FullMessage;
use config::Config;
use chat_lib::{chat::Message, settings::Settings};
use chat_lib::terminal::{capture_user_input, clear_terminal, print_messages, startup_message};
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

    //read in settings
    let settings = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .build()
        .unwrap();

    let username = settings.get_string("username").unwrap();

    //test
    //give output

    //make channel to communicate between the task that prints
    //and the tasks that receive messages
    let (tx, mut rx) = mpsc::channel::<FullMessage>(16);

    startup_message().await;
    
    let tx_mqtt = tx.clone();
    let tx_input = tx.clone();
 
    // //wait a bit to clear
    tokio::time::sleep(Duration::from_secs(3)).await;
    clear_terminal();
    tokio::time::sleep(Duration::from_secs(1)).await;

    //task to print messages
    tokio::spawn(async move {
        print_messages(rx).await
    });

    //task that handles mqtt connection
    // tokio::spawn(async move{
    //     mqtt_connect(settings, tx_mqtt).await
    // });

    //task that handles local input
    let handle = tokio::spawn(async move {
        capture_user_input(tx_input, &username).await;
    });
    handle.await.unwrap();


}
