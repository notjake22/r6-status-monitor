use crate::models::statuses::{
        PlatformStatusInfo,
        Status
    };
use crate::api::{
        status_check,
        status_check::StatusResponse
    };
use std::vec;
use tokio::time::{sleep, Duration};
use async_recursion::async_recursion;

pub enum State {
    Init,
    Run,
    SendWebhook
}

pub struct Control {
    cached_res: PlatformStatusInfo,
    current_state: State,
}

impl Control {
    pub fn new() -> Control {
        Control { 
            cached_res: PlatformStatusInfo { 
                statuses: Vec::new() 
            }, 
            current_state: State::Init 
        }
    }

    #[async_recursion]
    pub async fn run(&self) {
        let res = match status_check::check_status().await {
            Ok(res) => {
                println!("Response from api: {:?}", res);
                res
            }
            Err(error) => {
                panic!("Error making request to status api: {:?}", error)
            }
        };

        match self.current_state {
            State::Init => {
                let con = self.init(res).await;
                sleep(Duration::from_millis(3000)).await;
                con.run().await;
            }
            State::Run => {
                let statuses = PlatformStatusInfo::new(res);
                let con = self.run_check(statuses.statuses).await;
                con.run().await;
            }
            State::SendWebhook => {

            }
        }   
    }

    async fn init(&self, statuses: Vec<StatusResponse>) -> Control {
        let mut new_cache: Vec<Status> = Vec::new();
        for i in statuses {
            new_cache.push(Status 
                { 
                    platform: i.platform, 
                    status: i.status, 
                    impacted_features: i.impacted_features 
                }
            )
        }
        Control{cached_res: PlatformStatusInfo { statuses: new_cache }, current_state: State::Run}
    }

    async fn run_check(&self, res: Vec<Status>) -> Control {
        let mut vec_diff: Vec<Status> = vec![];
        for i in res {
            if !self.cached_res.statuses.contains(&i) {
                vec_diff.push(i);
            }
        }

        Control{cached_res: PlatformStatusInfo{statuses: vec_diff}, current_state: State::Run}
    }
}