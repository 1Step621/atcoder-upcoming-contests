use chrono::Duration;

use crate::{periodic::scrape, state::State};

pub async fn wait(state: State) -> ! {
    const DURATION: std::time::Duration = match Duration::days(1).to_std() {
        Ok(d) => d,
        Err(_) => panic!("Failed to convert duration"),
    };

    loop {
        scrape(&state).await;
        tokio::time::sleep(DURATION).await;
    }
}
