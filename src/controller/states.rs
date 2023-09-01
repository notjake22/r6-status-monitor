use crate::{
    discord::webhook::Webhook,
    models::statuses::{
        PlatformStatusInfo,
        Status
    },
    api::{
        status_check,
        status_check::StatusResponse
    }
};
use std::vec;
use tokio::time::{
    sleep, Duration
};
// use async_recursion::async_recursion;

pub enum State {
    Init,
    Run,
    SendWebhook
}

pub struct Control {
    cached_res: PlatformStatusInfo,
    current_state: State,
    webhook: String
}
      
impl Control {
    pub fn new(uri: String) -> Control {
        Control { 
            cached_res: PlatformStatusInfo { 
                statuses: Vec::new() 
            },
            current_state: State::Init,
            webhook: uri
        }
    }

    pub async fn run(&self) {
        let mut c = Control{
            cached_res: PlatformStatusInfo { 
                statuses: Vec::new()
            },
            current_state: State::Init,
            webhook: self.webhook.clone()
        };

        loop {
            let res = self.next(c).await;
            c = Control{
                cached_res: res.0.cached_res,
                current_state: res.1,
                webhook: res.0.webhook
            };
        }
    }

    async fn next(&self, control: Control) -> (Control, State) {
        let res = match status_check::check_status().await {
            Ok(res) => {
                // println!("Response from api: {:?}", res);
                res
            }
            Err(error) => {
                panic!("Error making request to status api: {:?}", error)
            }
        };

        match control.current_state {
            State::Init => {
                println!("Initializing");
                let con = self.init(res).await;
                sleep(Duration::from_millis(3000)).await;
                (con, State::Run)
            }
            State::Run => {
                println!("Checking Cache");
                let stats = PlatformStatusInfo::new_ref(&res);
                let diff_option = self.run_check(&stats.statuses);
                match diff_option {
                    None => {
                        sleep(Duration::from_millis(3000)).await;
                        (Control{
                            cached_res: PlatformStatusInfo { statuses: PlatformStatusInfo::new(res).statuses },
                            current_state: State::Run,
                            webhook: self.webhook.clone()
                        }, State::Run)
                    },
                    Some(_) => {
                        // chose to not use passed differential vector to just use the response from req 
                        (Control{
                            cached_res: PlatformStatusInfo::new(res),
                            current_state: State::Run,
                            webhook: self.webhook.clone()
                        }, State::SendWebhook)
                    }
                }
            }
            State::SendWebhook => {
                let hook = Webhook::new(control.cached_res, self.webhook.clone());

                match hook.send_webhook().await {
                    Ok(_) => {
                        println!("Webhook sent with status change");
                        sleep(Duration::from_millis(3000)).await;
                        (Control{
                            cached_res: PlatformStatusInfo { 
                                statuses: Vec::new() 
                            }, 
                            current_state: State::Init,
                            webhook: control.webhook.clone()
                        }, State::Init)
                    }
                    Err(error) => {
                        panic!("Error sending webhook for update status: {:?}", error);
                    }
                }
                
            }
        }   
    }

    async fn init(&self, statuses: Vec<StatusResponse>) -> Control {
        let mut new_cache: Vec<Status> = Vec::new();
        for i in statuses {
            new_cache.push(Status { 
                    platform: i.platform, 
                    status: i.status, 
                    impacted_features: i.impacted_features 
                }
            )
        }
        Control{
            cached_res: PlatformStatusInfo 
                { statuses: new_cache }, 
            current_state: State::Run,
            webhook: self.webhook.clone()
        }
    }

    fn run_check<'a>(&'a self, res: &'a Vec<Status>) -> Option<Vec<&Status>> {
        let mut vec_diff: Vec<&Status> = vec![];
        // dont really care too much about having nested loops just
        // because its only 3 items were going to be looping through each time
        for i in res {
            for c in &self.cached_res.statuses {
                if i.platform == c.platform {
                    if i.status != c.status {
                        vec_diff.push(i);
                    }
                }
            }
            // if !self.cached_res.statuses.contains(&i) {
            //     vec_diff.push(i);
            // }
        }

        if vec_diff.last().is_none() {
            None
        } else {
            Some(vec_diff)
        }
    }
}