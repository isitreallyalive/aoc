use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse2};

#[proc_macro_attribute]
pub fn solution(args: TokenStream, item: TokenStream) -> TokenStream {
    // parse function
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let original_body = &input_fn.block;

    input_fn.block = parse2(quote! {
        {
            let now = std::time::Instant::now();
            let result = (|| #original_body)();
            let elapsed = now.elapsed();
            println!("⏰ Solution took: {:.2?}", elapsed);
            result
        }
    })
    .unwrap();

    // return function
    quote! {
        #input_fn
    }
    .into()
}
