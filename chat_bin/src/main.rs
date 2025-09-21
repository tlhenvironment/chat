use config::Config;
use chat_lib::settings::Settings;
use chat_lib::terminal::clear_terminal;
use env_logger::Builder;
use log::info;
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

    //give output
    println!("--------------- Chat application ---------------");
    println!("\n \n");
    
    // //wait a bit to clear
    // tokio::time::sleep(Duration::from_secs(1)).await;
    // clear_terminal();

    //connect to MQTT server
    let _ = mqtt_connect(settings).await;

}
