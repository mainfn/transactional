use std::sync::Arc;

use super::context::Executor;

tokio::task_local! {
    pub static LOCAL_EXECUTOR: Arc<Executor>;
}
