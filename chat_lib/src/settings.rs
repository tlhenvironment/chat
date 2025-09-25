use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    mqtt_server_address: String,
    mqtt_server_port: usize,
    mqtt_server_preshared_key: String,
}