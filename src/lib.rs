use std::{
    borrow::Cow,
    process::{Child, Command},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use thiserror::Error;

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
    pub args: &'s [&'s str],
    handle: Option<Arc<Mutex<Child>>>,
}

impl<'s> Default for Chrome<'s>
{
    fn default() -> Self {
        Self {
            driver_path: "chromedriver".into(),
            server_url: "http://localhost:9515".into(),
            args: &[],
            handle: None,
        }
    }
}

impl<'s> Chrome<'s> {
    pub async fn spawn(&mut self, poll: Duration, timeout: Duration) -> Result<(), ChromeError> {
        let handle = Command::new(self.driver_path.to_string())
            .args(self.args)
            .spawn()
            .unwrap();
        let start = SystemTime::now();
        loop {
            let res = reqwest::get(format!("{}/status", self.server_url)).await;
            let req = match res {
                Ok(r) => r,
                Err(e) => {
                    if e.is_connect() {
                        if start.elapsed().unwrap() < timeout {
                            tokio::time::sleep(poll).await;
                            continue;
                        } else {
                            return Err(ChromeError::Timeout)
                        }
                    } else {
                        return Err(ChromeError::Request(e))
                    }
                },
            };
            let resp = req
                .json::<Health>()
                .await.unwrap();
            if resp.value.ready.unwrap_or(false) {
                self.handle = Some(Arc::new(Mutex::new(handle)));
                return Ok(());
            } else if start.elapsed().unwrap() < timeout {
                tokio::time::sleep(poll).await;
                continue;
            } else {
                return Err(ChromeError::Timeout)
            }
        }
    }
}

impl<'s> Drop for Chrome<'s> {
    fn drop(&mut self) {
        if let Some(handle_ptr) = &self.handle {
            if let Ok(mut handle) = handle_ptr.lock() {
                handle
                    .kill()
                    .unwrap_or_else(|_| panic!("failed to cleanup chromedriver process: {}", handle.id()));
            }
        }
    }
}
