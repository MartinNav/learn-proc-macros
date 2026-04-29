use core::convert::From;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    //eprintln!("TOKENS: {}", input);
    eprintln!("AST: {:#?}",ast);
    let expanded = quote!{
        
        #[automatically_derived]
        impl #name {
            pub fn builder()-> #name {

            }
        }

    };
    TokenStream::from(expanded)
}
