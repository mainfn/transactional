use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn transactional(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let expanded = quote! {
        pub async fn #fn_name() -> Result<(), Box<dyn std::error::Error>> {
            let start_time = std::time::Instant::now();
            let executor = std::sync::Arc::new(CONTEXT.new_executor().await);

            LOCAL_EXECUTOR
                .scope(executor.clone(), async {
                    executor.begin().await;
                    let result = (async { #fn_block }).await;

                    match result {
                        Ok(_) => executor.commit().await,
                        Err(_) => executor.rollback().await,
                    }

                    let end_time = std::time::Instant::now();
                    println!(
                        "[Service Logic - Conn id {}, TX id {}, Tx State: {:?}] elapsed time: {:?}",
                        executor.conn_id().await,
                        executor.tx_id().await,
                        executor.tx_state().await,
                        end_time.duration_since(start_time)
                    );
                    executor.release_conn().await;

                    result
                })
                .await
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn repository(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let expanded = quote! {
        pub fn #fn_name() -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
            LOCAL_EXECUTOR.with(|executor| {
                let executor = executor.clone();
                Box::pin(async move {
                    let start_time = std::time::Instant::now();
                    let result = (async { #fn_block }).await;
                    let end_time = std::time::Instant::now();
                    println!(
                        "[Repository Logic - Conn id {}, TX id {}] elapsed time: {:?}",
                        executor.conn_id().await,
                        executor.tx_id().await,
                        end_time.duration_since(start_time)
                    );
                    result
                })
            })
        }
    };

    TokenStream::from(expanded)
}
