use std::{fmt::Alignment, io, net::SocketAddr, time::Duration};

use config::Config;
use log::{debug, info};
use rumqttc::{tokio_rustls::rustls::crypto::cipher::MessageEncrypter, AsyncClient, EventLoop, Incoming, MqttOptions, NetworkOptions, QoS};
use tokio::{net::{self}, sync::mpsc::{Receiver, Sender}, task, time};
use chat_lib::chat::{FullMessage, Message};

pub async fn mqtt_connect(config: Config, tx: Sender<FullMessage>, rx: Receiver<Message>,
        random_hash_state: String) -> ! {
    let mqtt_server_address = config.get_string("mqtt_server_address").unwrap();
    let mqtt_server_port = config.get_int("mqtt_server_port").unwrap() as u16;
    let mqtt_server = format!("{}:{}", mqtt_server_address, mqtt_server_port);
    
    debug!("test: {}", mqtt_server);

    let mut mqttoptions = MqttOptions::new(random_hash_state, &mqtt_server_address, mqtt_server_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    task::spawn(async move {
        mqtt_eventloop(eventloop, tx).await;
    });

    task::spawn(async move {
        mqtt_client(client, rx).await;
    });

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn mqtt_eventloop(mut eventloop: EventLoop, tx: Sender<FullMessage>) {
    loop {
        let notification = match eventloop.poll().await {
            Ok(x) => x,
            Err(e) => {
                println!("Connection error: {}, \n Trying to reconnect...", e);
                time::sleep(Duration::from_secs(5)).await;
                continue
            },
        };
        debug!("Received = {:?}", notification);

        let packet = match notification {
            rumqttc::Event::Outgoing(outgoing) => {
                debug!("outgoing: {:?}", outgoing);
                continue;
            },
            rumqttc::Event::Incoming(packet) => packet,
        };

        let publish = match packet {
            rumqttc::Packet::Publish(publish) => publish,
            _ => continue,
        };

        debug!("topic:{}", &publish.topic);

        let message: Message = match bincode::decode_from_slice(&publish.payload, bincode::config::standard()) {
            Ok(x) => x.0,
            Err(e) => {
                debug!("Deserialization error {}", e);
                continue
            }
        };

        let full_message = FullMessage::new(message, chat_lib::chat::ChatAlignment::Left);

        tx.send(full_message).await.unwrap();

        // match message.chat_print(chat_lib::chat::ChatAlignment::Right) {
        //     Ok(_) => (),
        //     Err(e) => {
        //         println!("{:?}", e);
        //         continue;
        //     },
        // }
    }
}

async fn mqtt_client(client: AsyncClient, mut rx: Receiver<Message>) {
    debug!("here");
    client.subscribe("chat/#", QoS::ExactlyOnce).await.unwrap();

    loop {
        let message = rx.recv().await.unwrap();

        debug!("mqtt client task received message {:?}", message);

        let encoded_message = bincode::encode_to_vec(message, bincode::config::standard()).unwrap();

        match client.publish("chat/1", rumqttc::QoS::AtMostOnce, false, encoded_message).await {
            Ok(_) => continue,
            Err(_) => println!("-----OFFLINE, trying to reconnect-------"),
        }
    }
}

