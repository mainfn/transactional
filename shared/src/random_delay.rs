use std::time::Duration;

use rand::Rng;
use tokio::time::sleep;

pub async fn random_delay(min_millis: u64, max_millis: u64) {
    let mut rng = rand::thread_rng();

    let random_secs = rng.gen_range(min_millis..max_millis);
    let random_duration = Duration::from_millis(random_secs);

    sleep(random_duration).await;
}
