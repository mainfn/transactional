use std::{sync::Arc, time::Duration};

use rand::{
    rngs::{SmallRng, StdRng},
    thread_rng, Rng, RngCore, SeedableRng,
};
use tokio::{
    sync::{Mutex, OnceCell},
    task::{futures, JoinSet},
    time::{interval, sleep},
    try_join,
};
use tokio_stream::{wrappers::IntervalStream, StreamExt};

use crate::service::{self, service_logic};
pub async fn run() {
    let mut interval = IntervalStream::new(interval(Duration::from_millis(1)));

    while let Some(_) = interval.next().await {
        for i in 0..50 {
            let r = try_join!(service_logic(), service_logic(), service_logic());
        }
    }
}
