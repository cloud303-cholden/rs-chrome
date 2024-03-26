use std::time::Duration;

use chrome::Chrome;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Requires the `chromedriver` binary to be at the below path.
    let mut chrome = Chrome {
        driver_path: "/usr/local/bin/chromedriver",
        ..Default::default()
    };
    chrome.spawn(
        Duration::from_secs(1), // Health poll interval
        Duration::from_secs(5), // Health poll timeout
    ).await?;

    Ok(())
}


