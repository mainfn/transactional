use shared::random_delay::random_delay;
use tokio::time::Instant;

use crate::tx::local_tx::LOCAL_EXECUTOR;

pub async fn repository_logic() -> Result<(), ()> {
    LOCAL_EXECUTOR
        .with(|executor| {
            // must clone `local_tx`
            // because a lifetime of `local_tx` must be extended to the end of the function
            // or `local_tx` will be destroyed after the function returns
            // and the `tx` will be dropped
            // so we need to clone `local_tx` to extend its lifetime
            let executor = executor.clone();
            Box::pin(async move {
                let start_time = Instant::now();
                random_delay(10, 100).await;

                let end_time = Instant::now();
                println!(
                    "[Repository Logic - Conn id {}, TX id {}] elasped time: {:?}",
                    executor.conn_id().await,
                    executor.tx_id().await,
                    end_time.duration_since(start_time)
                );
                Ok(())
            })
        })
        .await
}
