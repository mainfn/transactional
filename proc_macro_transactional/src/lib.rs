// #[proc_macro_attribute]
// pub fn transactional(args: TokenStream, input: TokenStream) -> TokenStream {
//     // parse the function signature
//     let input = parse_macro_input!(input as ItemFn);
//     let fn_name = &input.sig.ident;
//     let fn_body = &input.block;
//     let fn_arguments = &input.sig.inputs;
//     let fn_return_type = &input.sig.output;

//     // generate the function body
//     let output = quote!{
//         async fn #fn_name(#fn_arguments) #fn_return_type {
//             // get transaction from tokio thread local

//         }
//     }
// }
