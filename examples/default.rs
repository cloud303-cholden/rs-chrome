use std::time::Duration;

use chrome::Chrome;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chrome = Chrome::default();
    chrome.spawn(
        Duration::from_secs(1), // Poll interval
        Duration::from_secs(5), // Timeout
    ).await?;

    Ok(())
}

