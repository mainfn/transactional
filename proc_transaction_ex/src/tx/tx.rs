use shared::fake_connection::FakeConn;

use super::context::Context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxStatus {
    Initial,
    Active,
    Committed,
    RolledBack,
}

#[derive(Debug, Clone)]
pub struct Tx {
    id: usize,
    conn: FakeConn,
    status: TxStatus,
}

impl Tx {
    pub fn new(id: usize, conn: FakeConn) -> Self {
        Self {
            id,
            conn,
            status: TxStatus::Initial,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn conn(&self) -> &FakeConn {
        &self.conn
    }

    pub fn status(&self) -> &TxStatus {
        &self.status
    }

    pub async fn begin(&mut self) {
        self.conn.send().await;
        self.status = TxStatus::Active;
    }

    pub async fn commit(&mut self) {
        self.conn.send().await;
        self.conn.recv().await;
        self.status = TxStatus::Committed;
    }

    pub async fn rollback(&mut self) {
        self.conn.send().await;
        self.conn.recv().await;
        self.status = TxStatus::RolledBack;
    }
}

unsafe impl Send for Tx {}
unsafe impl Sync for Tx {}
