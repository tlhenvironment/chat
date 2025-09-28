use config::Config;
use chat_lib::{chat::Message, settings::Settings};
use chat_lib::terminal::{capture_user_input, clear_terminal};
use env_logger::Builder;
use log::info;
use tokio::io::{self, AsyncBufReadExt as _, BufReader};
use std::fmt::Alignment;
use std::{env, thread, time::{self, Duration}};

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

    //test
    let t = Message::new("lollo".to_string(), "aweweggwe".to_string());
    t.chat_print(chat_lib::chat::ChatAlignment::Left).unwrap();
    t.chat_print(chat_lib::chat::ChatAlignment::Right).unwrap();



    // tokio::time::sleep(Duration::from_secs(100)).await;

    //give output
    println!("--------------- Chat application ---------------");
    println!("\n \n");
    
    // //wait a bit to clear
    tokio::time::sleep(Duration::from_secs(5)).await;
    clear_terminal();
    tokio::time::sleep(Duration::from_secs(1)).await;

    //connect to MQTT server
    tokio::spawn(async move{
        mqtt_connect(settings).await
    });

    //input loop
    let handle = tokio::spawn(async move {
        capture_user_input().await;
    });

    handle.await.unwrap();


}
