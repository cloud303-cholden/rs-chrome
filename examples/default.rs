use std::time::Duration;

use chrome::Chrome;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chrome = Chrome::default();
    chrome.spawn(
        Duration::from_secs(1), // Health poll interval
        Duration::from_secs(5), // Health poll timeout
    ).await?;

    Ok(())
}

