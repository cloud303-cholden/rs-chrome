### Chromedriver
Dead-simple library to launch a subprocess of the `chromedriver`. Use at your own risk.
### Usage
This crate is only hosted on GitHub for now.
```sh
cargo add --git https://github.com/cloud303-cholden/rs-chrome.git
```
### Configuration
The `Chrome` struct has the following public `Default` implementation.
```rust
Chrome {
    driver_path: "chromedriver".into(),
    server_url: "http://localhost:9515".into(),
    args: &[],
}
```
### Example
You can run the below example with `cargo run --example default`. The current behavior is for the `chromedriver` process to get cleaned up automatically when `Chrome` is dropped. If this is not behaving as expected, try first running `kill $(pidof chromedriver)`.
```rust
use std::time::Duration;

use chrome::Chrome;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chrome = Chrome::default();

    // Spawns the process and returns a result based on the
    // chromedriver server's health
    chrome.spawn(
        Duration::from_secs(1), // Health poll interval
        Duration::from_secs(5), // Health poll timeout
    ).await?;

    // Do stuff

    Ok(())
}
```
