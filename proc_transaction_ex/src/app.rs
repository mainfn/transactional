use std::time::Duration;

use tokio::{time::interval, try_join};
use tokio_stream::{wrappers::IntervalStream, StreamExt};

use crate::service::service_logic;
pub async fn run() {
    let mut interval = IntervalStream::new(interval(Duration::from_millis(1)));

    while let Some(_) = interval.next().await {
        for i in 0..50 {
            let r = try_join!(service_logic(), service_logic(), service_logic());
        }
    }
}
