use std::{io, net::SocketAddr, time::Duration};

use config::Config;
use log::{debug, info};
use rumqttc::{EventLoop, Incoming, AsyncClient, MqttOptions, NetworkOptions, QoS};
use tokio::{net::{self}, task, time};
use chat_lib::net::Message;

pub async fn mqtt_connect(config: Config) -> ! {
    let mqtt_server_address = config.get_string("mqtt_server_address").unwrap();
    let mqtt_server_port = config.get_int("mqtt_server_port").unwrap() as u16;
    let mqtt_server = format!("{}:{}", mqtt_server_address, mqtt_server_port);
    
    println!("test: {}", mqtt_server);

    loop {
    
        // let socket_addrs: Vec<SocketAddr> = match net::lookup_host(&mqtt_server).await {
        //     Ok(x) => {
        //         x.collect()
        //     },
        //     Err(e) => {
        //         println!("error: {:?}", e);
        //         tokio::time::sleep(Duration::from_secs(1)).await;
        //         continue
        //     }
        // };

        // println!("result: {:?}", socket_addrs);

        //connect to mqtt
        let mut mqttoptions = MqttOptions::new("chat", &mqtt_server_address, mqtt_server_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        client.subscribe("chat/#", QoS::ExactlyOnce).await.unwrap();

        task::spawn(async move {
            mqtt_eventloop(eventloop).await;
        });

        task::spawn(async move {
            mqtt_client(client).await;
        });
        
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

async fn mqtt_eventloop(mut eventloop: EventLoop) {
    loop {
        let notification = match eventloop.poll().await {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                break
            },
        };
        println!("Received = {:?}", notification);

        let packet = match notification {
            rumqttc::Event::Outgoing(outgoing) => {
                println!("outgoing: {:?}", outgoing);
                continue;
            },
            rumqttc::Event::Incoming(packet) => packet,
        };

        let publish = match packet {
            rumqttc::Packet::Publish(publish) => publish,
            _ => continue,
        };

        println!("topic:{}", &publish.topic);

        let message: Message = match bincode::decode_from_slice(&publish.payload, bincode::config::standard()) {
            Ok(x) => x.0,
            Err(e) => {
                println!("{}", e);
                continue
            }
        };

        println!("message: {}", message);

    }
}

async fn mqtt_client(client: AsyncClient) {
    loop {
        println!("test client");
        let message = Message::new("test".to_string(), "tt".to_string());
        let encoded_message = bincode::encode_to_vec(message, bincode::config::standard()).unwrap();
        client.publish("chat/1", rumqttc::QoS::AtMostOnce, false, encoded_message).await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}