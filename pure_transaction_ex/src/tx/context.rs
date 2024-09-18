use lazy_static::lazy_static;
use shared::fake_connection::{FakeConn, FakeConnPool};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{
    seq_gen::SeqGen,
    tx::{Tx, TxStatus},
};

const MAX_CONNS: usize = 50;

lazy_static! {
    pub static ref CONTEXT: Arc<Context> = Arc::new(Context::new());
    pub static ref CONN_POOL: Arc<FakeConnPool> = Arc::new(FakeConnPool::new(MAX_CONNS));
}
#[derive(Debug)]
pub struct Context {
    seq_gen: SeqGen,
}

impl Context {
    pub fn new() -> Self {
        Self {
            seq_gen: SeqGen::new(),
        }
    }

    async fn conn(&self) -> FakeConn {
        CONN_POOL.acquire().await
    }

    pub async fn new_executor(&self) -> Executor {
        Executor::new(Tx::new(self.seq_gen.next(), self.conn().await))
    }
}

pub struct Executor {
    tx: Mutex<Tx>,
}

impl Executor {
    pub fn new(tx: Tx) -> Self {
        Self { tx: Mutex::new(tx) }
    }

    pub async fn begin(&self) {
        let mut tx = self.tx.lock().await;
        if tx.status() != &TxStatus::Initial {
            panic!("Transaction is not in Initial state");
        }
        tx.begin().await;
    }

    pub async fn commit(&self) {
        let mut tx = self.tx.lock().await;
        if tx.status() != &TxStatus::Active {
            panic!("Transaction is not in Active state");
        }
        tx.commit().await;
    }

    pub async fn rollback(&self) {
        let mut tx = self.tx.lock().await;
        if tx.status() != &TxStatus::Active {
            panic!("Transaction is not in Active state");
        }
        tx.rollback().await;
    }
    pub async fn tx_id(&self) -> usize {
        self.tx.lock().await.id()
    }
    pub async fn tx_state(&self) -> TxStatus {
        self.tx.lock().await.status().to_owned()
    }
    pub async fn conn_id(&self) -> usize {
        self.tx.lock().await.conn().id()
    }

    pub async fn release_conn(&self) {
        CONN_POOL
            .release(self.tx.lock().await.conn().to_owned())
            .await;
    }
}
unsafe impl Send for Context {}
unsafe impl Sync for Context {}
unsafe impl Send for Executor {}
unsafe impl Sync for Executor {}
