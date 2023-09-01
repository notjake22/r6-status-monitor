use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use reqwest::{Error, header};
use reqwest;
use crate::models::statuses::PlatformStatusInfo;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookRoot {
    pub content: Value,
    pub embeds: Vec<Embed>,
    pub attachments: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    pub title: String,
    pub color: i64,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

pub struct Webhook {
    status: PlatformStatusInfo,
    webhook_url: String
    // embeds: HookRoot
}

impl Webhook {
    pub fn new(stats: PlatformStatusInfo, web_uri: String) -> Webhook {
        Webhook { 
            status: stats, 
            webhook_url: web_uri 
        }
    }

    pub async fn send_webhook(&self) -> Result<(), Error> {
        let payload = self.construct_webhook_payload();

        let client = reqwest::Client::new();
        let res = client.post(self.webhook_url.clone())
            .json(&payload)
            .header(header::ACCEPT, "application/json")
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        res.error_for_status()?;

        Ok(())
    }

    fn construct_webhook_payload(&self) -> HookRoot {
        let mut ebs: Vec<Embed> = Vec::new();

        for e in self.status.statuses.iter() {
            let mut feild_vec: Vec<Field> = Vec::new();

            feild_vec.push(Field { 
                name: "Platform".to_string(), 
                value: e.platform.clone(), 
                inline: Some(true) 
            });
            feild_vec.push(Field { 
                name: "Status".to_string(), 
                value: e.status.clone(), 
                inline: Some(true) 
            });

            let mut features: String = "".to_string();
            if e.impacted_features.last().is_none() {
                features = "None".to_string();
            } else {
                for f in &e.impacted_features {
                    features = format!("{}\n{}", &features, f);
                }
            }

            feild_vec.push(Field { 
                name: "Impacted Features".to_string(), 
                value: features, 
                inline: Some(true) 
            });

            ebs.push(Embed { 
                title: "Official R6S Server Status Update".to_string(), 
                color: 7203260, 
                fields: feild_vec 
            })
        }

        let attch: Vec<Value> = Vec::new();
        HookRoot { 
            content: Value::Null, 
            embeds: ebs, 
            attachments: attch 
        }
    }
} 