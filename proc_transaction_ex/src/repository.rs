use local_transactional::repository;
use shared::random_delay::random_delay;
use tokio::time::Instant;

use crate::tx::local_tx::LOCAL_EXECUTOR;

#[repository]
pub async fn repository_logic() -> Result<(), ()> {
    random_delay(10, 100).await;
    Ok(())
}
