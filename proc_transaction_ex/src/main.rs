use proc_transaction_ex::app;

#[tokio::main]
async fn main() {
    app::run().await;
}
