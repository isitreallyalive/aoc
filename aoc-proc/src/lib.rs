use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemFn, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input, parse2,
};

struct SolutionArgs {
    out: Vec<LitStr>,
}

impl Parse for SolutionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut out = Vec::new();
        while !input.is_empty() {
            out.push(input.parse()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { out })
    }
}

#[proc_macro_attribute]
pub fn solution(args: TokenStream, item: TokenStream) -> TokenStream {
    // parse function
    let SolutionArgs { out } = parse_macro_input!(args as SolutionArgs);
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let statements = &input_fn.block.stmts;
    let out_stmts = out.iter().map(|fmt| {
        quote! {
            println!("ℹ️  {}", format!(#fmt));
        }
    });

    // modify function with timing logic and output
    input_fn.block = parse2(quote! {
        {
            const INPUT: &str = include_str!("input.txt");
            let now = std::time::Instant::now();
            #(#statements)*
            let elapsed = now.elapsed();
            #(#out_stmts)*
            println!();
            println!("⏰ Solution took: {:.2?}", elapsed);
        }
    })
    .unwrap();

    // return function
    quote! {
        #input_fn
    }
    .into()
}
