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
}
      
impl Control {
    pub fn new() -> Control {
        Control { 
            cached_res: PlatformStatusInfo { 
                statuses: Vec::new() 
            }
        }
    }

    #[async_recursion]
    pub async fn run(&self) {
        let mut state = State::Init;
        let mut c = Control{
            cached_res: PlatformStatusInfo { 
                statuses: self.cached_res.statuses 
            }
        };

        loop {
            let res = self.next(state).await;
            state = res.1;
            c = res.0;
        }
    }

    async fn next(&self, state: State) -> (Control, State) {
        let res = match status_check::check_status().await {
            Ok(res) => {
                println!("Response from api: {:?}", res);
                res
            }
            Err(error) => {
                panic!("Error making request to status api: {:?}", error)
            }
        };

        match state {
            State::Init => {
                let con = self.init(res).await;
                sleep(Duration::from_millis(3000)).await;
                (con, State::Run)
            }
            State::Run => {
                let statuses = PlatformStatusInfo::new(res);
                let con = self.run_check(statuses.statuses).await;
                match con.0 {
                    None => {
                        sleep(Duration::from_millis(3000)).await;
                        (con.1, State::Run)
                    },
                    Some(status_diffs) => {
                        // todo impl sending discord webhooks
                        sleep(Duration::from_millis(3000)).await;
                        (con.1, State::SendWebhook)
                    }
                }
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

    async fn run_check(&self, res: Vec<Status>) -> (Option<&Vec<Status>>, Control) {
        let mut vec_diff: Vec<&Status> = vec![];
        for i in &res {
            if !self.cached_res.statuses.contains(&i) {
                vec_diff.push(i);
            }
        }

        let mut control = Control{cached_res: PlatformStatusInfo{statuses: res}, current_state: State::Run};

        if vec_diff.last().is_none() {
            (None, control)
        } else {
            control.current_state = State::SendWebhook;
            (Some(&vec_diff), control)
        }
    }
}