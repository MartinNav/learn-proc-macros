use core::convert::From;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    //eprintln!("TOKENS: {}", input);
    eprintln!("AST: {:#?}",ast);
    let bident = syn::Ident::new(&format!("{}Builder", name), name.span());
    let expanded = quote!{
        pub struct #bident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }
        
        impl #name {
            pub fn builder()-> #bident {
                #bident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

    };
    TokenStream::from(expanded)
}
