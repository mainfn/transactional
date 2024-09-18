use std::sync::Arc;

use local_transactional::transactional;
use tokio::{time::Instant, try_join};

use crate::{
    repository::repository_logic,
    tx::{context::CONTEXT, local_executor::LOCAL_EXECUTOR},
};

#[transactional]
pub async fn service_logic() -> Result<(), ()> {
    // transaction management logic is automatically handled by the proc macro
    // so we can just call the repository logic directly
    // and the transaction will be automatically committed or rolled back
    // without additional code
    //
    // So we can focus on the business logic
    // without worrying about the transaction management
    //
    // This is called separation of concerns
    // and it is a good practice in software development
    //
    // Specifically this kind of segregation is called AOP(aspect-oriented programming)
    // https://en.wikipedia.org/wiki/Aspect-oriented_programming
    // AOP is a programming paradigm that aims to increase modularity by allowing the separation of cross-cutting concerns
    let result = try_join!(repository_logic(), repository_logic(), repository_logic());
    Ok(())
}
