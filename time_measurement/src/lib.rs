use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn time_measurement(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    /**
     * if async function is executed,
     * the function body is executed on the runtime like tokio.
     */
    let is_sync = input.sig.asyncness.is_none();

    let timing_code = quote! {
        let start_time = std::time::Instant::now();
        #fn_block
        let end_time = std::time::Instant::now();
        println!("Execution time: {:?}", end_time.duration_since(start_time));
    };

    let expanded = if is_sync {
        quote! {
            pub fn #fn_name() {
                #timing_code
            }
        }
    } else {
        quote! {
            pub async fn #fn_name() {
                #timing_code
            }
        }
    };

    TokenStream::from(expanded)
}
