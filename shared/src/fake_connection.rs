use std::{collections::VecDeque, sync::Arc};

use tokio::sync::{Mutex, Notify};

use crate::random_delay;

#[derive(Debug, Clone)]
pub struct FakeConn {
    pub id: usize,
}

impl FakeConn {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub async fn send(&self) {
        // send a message to the database
        random_delay::random_delay(5, 50).await;
    }

    pub async fn recv(&self) {
        // receive a message from the database
        random_delay::random_delay(10, 100).await;
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
pub struct FakeConnPool {
    connections: Mutex<VecDeque<FakeConn>>,
    notify: Notify,
    capacity: usize,
}

impl FakeConnPool {
    // create a new `FakePool` with the given capacity
    pub fn new(capacity: usize) -> Self {
        let connections = (0usize..capacity)
            .map(FakeConn::new)
            .collect::<VecDeque<_>>();
        Self {
            connections: Mutex::new(connections),
            notify: Notify::new(),
            capacity,
        }
    }

    // acquire a connection from the pool
    pub async fn acquire(&self) -> FakeConn {
        loop {
            // block needs to prevent deadlocks(narrow critical section)
            {
                // lock the connections and pop the first one
                let mut connections = self.connections.lock().await;
                if let Some(connection) = connections.pop_front() {
                    return connection;
                }
                // automatically unlock the connections
            }
            // notified() will be notified when a connection is released
            self.notify.notified().await;
        }
    }

    // release a connection to the pool
    pub async fn release(&self, connection: FakeConn) {
        {
            // lock the connections and push the connection to the end
            let mut connections = self.connections.lock().await;
            connections.push_back(connection);
        }
        // notify one of the blocked `acquire` calls
        self.notify.notify_one();
    }
}

#[cfg(test)]
mod tests {

    use futures::future::join_all;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use tokio::{
        task,
        time::{sleep, Instant},
    };

    use super::*;

    #[tokio::test]
    async fn test_acquire_release() {
        let pool = Arc::new(FakeConnPool::new(1000));

        let mut handles = vec![];
        for i in 0..10000 {
            let pool = pool.clone();
            let handle = task::spawn(async move {
                // generate a random sleep duration between 1-10ms before starting
                let mut rng = StdRng::from_entropy();
                let initial_sleep_duration = rng.gen_range(1..=10);
                tokio::time::sleep(tokio::time::Duration::from_millis(initial_sleep_duration))
                    .await;

                // for calculating the elapsed time
                let start_time = Instant::now();
                // set random sleep duration til the release
                let mut rng = StdRng::from_entropy();
                let sleep_duration = rng.gen_range(10..=50);

                // connection acquisition and release
                let conn = pool.acquire().await;
                let conn_id = conn.id;
                println!("[connection acquired] conn id: {}, {}th", conn_id, i + 1);
                sleep(tokio::time::Duration::from_millis(sleep_duration)).await;
                pool.release(conn).await;

                // calculate the elapsed time
                let end_time = Instant::now();
                let elasped_time = end_time - start_time;
                println!(
                    "[connection released] id: {}, {}th, elasped: {}ms",
                    conn_id,
                    i + 1,
                    elasped_time.as_millis()
                );
            });
            handles.push(handle);
        }

        // wait for all tasks to complete
        join_all(handles).await;
    }
}
