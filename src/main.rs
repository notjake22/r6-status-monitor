mod api;
mod discord;
mod models;
mod controller;

use controller::states::Control;
use std::{fs::File, io::Read};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Settings {
    webhook: String
}

#[tokio::main]
async fn main() {
    // test
    // sleep(Duration::from_millis(1000)).await;
    let mut file = match File::open("./settings.json"){
        Ok(file) => file,
        Err(error) => {
            panic!("Error while opening file: {:?}", error);
        }
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let settings: Settings = serde_json::from_str(&data).expect("Invalid JSON file");

    let conn = Control::new(settings.webhook);
    conn.run().await;
}
