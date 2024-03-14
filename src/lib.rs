use std::{
    borrow::Cow,
    time::{Duration, SystemTime},
};

use thiserror::Error;
use tokio::process::{Command, Child};

#[derive(Debug, Error)]
pub enum ChromeError {
    #[error("timeout reached")]
    Timeout,
    #[error("request error")]
    Request(#[from] reqwest::Error),
}

#[derive(Debug, serde::Deserialize)]
pub struct Health {
    pub value: Value,
}

#[derive(Debug, serde::Deserialize)]
pub struct Value {
    pub ready: Option<bool>,
}

#[derive(Debug)]
pub struct Chrome<'s> {
    pub driver_path: Cow<'s, str>,
    pub server_url: Cow<'s, str>,
}

impl<'s> Default for Chrome<'s>
{
    fn default() -> Self {
        Self {
            driver_path: "chromedriver".into(),
            server_url: "http://localhost:9515".into(),
        }
    }
}

impl<'s> Chrome<'s> {
    pub async fn spawn(self, poll: Duration, timeout: Duration) -> Result<Child, ChromeError> {
        let handle = Command::new(self.driver_path.to_string())
            .spawn()
            .unwrap();
        let start = SystemTime::now();
        loop {
            let res = reqwest::get(format!("{}/status", self.server_url)).await;
            let req = match res {
                Ok(r) => r,
                Err(e) => {
                    if e.is_connect() {
                        tokio::time::sleep(poll).await;
                        continue
                    } else {
                        return Err(ChromeError::Request(e))
                    }
                },
            };
            let resp = req
                .json::<Health>()
                .await.unwrap();
            if resp.value.ready.unwrap_or(false) {
                return Ok(handle);
            } else if start.elapsed().unwrap() < timeout {
                tokio::time::sleep(poll).await;
                continue;
            } else {
                return Err(ChromeError::Timeout)
            }
        }
    }
}
