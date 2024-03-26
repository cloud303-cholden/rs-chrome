use std::{
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
    pub driver_path: &'s str,
    pub server_url: &'s str,
    pub args: &'s [&'s str],
    pub handle: Option<ChromeProcess>,
}

impl<'s> Default for Chrome<'s>
{
    fn default() -> Self {
        Self {
            driver_path: "chromedriver",
            server_url: "http://localhost:9515",
            args: &[],
            handle: None,
        }
    }
}

impl<'s> Chrome<'s> {
    pub async fn spawn(&mut self, poll: Duration, timeout: Duration) -> Result<(), ChromeError> {
        let handle = Command::new(self.driver_path)
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
                self.handle = Some(ChromeProcess{
                    inner: Arc::new(Mutex::new(handle)),
                });
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

#[derive(Debug)]
pub struct ChromeProcess {
    pub inner: Arc<Mutex<Child>>,
}

impl Drop for ChromeProcess {
    fn drop(&mut self) {
        if let Ok(mut handle) = self.inner.lock() {
            handle.kill().unwrap();
        }
    }
}
