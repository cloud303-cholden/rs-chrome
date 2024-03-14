### Chromedriver
Dead-simple library to launch a subprocess of the `chromedriver`. Use at your own risk.
### Usage
This crate is only hosted on GitHub for now.
```sh
cargo add --git https://github.com/cloud303-cholden/rs-chrome.git
```
### Configuration
The `Chrome` struct has the following `Default` implementation.
```rust
Chrome {
    driver_path: "chromedriver".into(),
    server_url: "http://localhost:9515".into(),
}
```
### Example
You can run the below example with `cargo run --example default`. The current behavior is for the `chromedriver` process to outlive the application unless cleaned up with `kill()`. If you forget to add this, try `kill $(pidof chromedriver)`.
```rust
use std::time::Duration;

use chrome::Chrome;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chrome = Chrome::default();
    let mut handle = chrome.spawn(
        Duration::from_secs(1), // Poll interval
        Duration::from_secs(5), // Timeout
    ).await?;

    handle
        .kill()
        .await?;

    Ok(())
}
```
