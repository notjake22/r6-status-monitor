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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn run_checks() {
        let res: Vec<api::status_check::StatusResponse> = vec![api::status_check::StatusResponse{
            app_id: String::from(""),
            category: String::from(""),
            mdm: String::from(""),
            name: String::from("Rainbow Six Siege - PC - LIVE"),
            platform: String::from("PC"),
            status: String::from("Online"),
            space_id: String::from(""),
            maintenance: serde_json::Value::Null,
            impacted_features: vec![]
        }];
        let res_2: Vec<api::status_check::StatusResponse> = vec![api::status_check::StatusResponse{
            app_id: String::from(""),
            category: String::from(""),
            mdm: String::from(""),
            name: String::from("Rainbow Six Siege - PC - LIVE"),
            platform: String::from("PC"),
            status: String::from("Degraded"),
            space_id: String::from(""),
            maintenance: serde_json::Value::Null,
            impacted_features: vec![]
        }];
        let stats_1 = models::statuses::PlatformStatusInfo::new_ref(&res);
        let stats_2 = models::statuses::PlatformStatusInfo::new(res);
        let stats_3 = models::statuses::PlatformStatusInfo::new(res_2);
        let con_check = Control{
            cached_res: stats_1,
            current_state: controller::states::State::Run,
            webhook: String::from("")
        };
        assert_eq!(None, con_check.run_check(&stats_2.statuses));
        assert_eq!(Some(vec![&models::statuses::Status{
            impacted_features: vec![],
            platform: String::from("PC"),
            status: String::from("Degraded")
        }]), con_check.run_check(&stats_3.statuses))
    }
}