use serde::Serialize;
use serde::Deserialize;
use crate::api::status_check::StatusResponse;

#[derive(Deserialize, Serialize, PartialEq)]
pub struct PlatformStatusInfo {
    pub statuses: Vec<Status>
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Status {
    pub platform: String,
    pub status: String, 
    pub impacted_features: Vec<String>
}

impl PlatformStatusInfo {
    pub fn new(res: Vec<StatusResponse>) -> PlatformStatusInfo {
        let mut new_statuses: Vec<Status> = Vec::new();
        for status in res {
            new_statuses.push(Status { 
                platform: status.platform, 
                status: status.status, 
                impacted_features: status.impacted_features 
            })
        }

        PlatformStatusInfo{
            statuses: new_statuses
        }
    }

    pub fn new_ref(res: &Vec<StatusResponse>) -> PlatformStatusInfo {
        let mut new_statuses: Vec<Status> = Vec::new();
        for status in res {
            new_statuses.push(Status { 
                platform: status.platform.clone(), 
                status: status.status.clone(), 
                impacted_features: status.impacted_features.clone() 
            })
        }

        PlatformStatusInfo{
            statuses: new_statuses
        }
    }
}