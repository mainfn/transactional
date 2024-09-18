use std::sync::Arc;

use tokio::{time::Instant, try_join};

use crate::{
    repository::repository_logic,
    tx::{context::CONTEXT, local_executor::LOCAL_EXECUTOR},
};

pub async fn service_logic() -> Result<(), ()> {
    let start_time = Instant::now();
    let executor = Arc::new(CONTEXT.new_executor().await);

    LOCAL_EXECUTOR
        .scope(executor.clone(), async {
            executor.begin().await;
            let result = try_join!(repository_logic(), repository_logic(), repository_logic());

            match result {
                Ok(_) => executor.commit().await,
                Err(_) => executor.rollback().await,
            }

            let end_time = Instant::now();
            println!(
                "[Service Logic - Conn id {}, TX id {}, Tx State: {:?}] elasped time: {:?}",
                executor.conn_id().await,
                executor.tx_id().await,
                executor.tx_state().await,
                end_time.duration_since(start_time)
            );
            executor.release_conn().await;
        })
        .await;

    Ok(())
}
