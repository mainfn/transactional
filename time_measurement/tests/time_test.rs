use time_measurement::time_measurement;

#[time_measurement]
fn sync_function() {
    let sum: u32 = (1..=100).sum();
    println!("Sum: {}", sum);
}

#[time_measurement]
async fn async_function() {
    let sum: u32 = (1..=100).sum();
    println!("Sum: {}", sum);
}

#[tokio::test]
async fn test_sync_function() {
    sync_function();
}

#[tokio::test]
async fn test_async_function() {
    async_function().await;
}
