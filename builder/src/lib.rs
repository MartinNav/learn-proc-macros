use core::convert::From;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    //eprintln!("TOKENS: {}", input);
    //eprintln!("AST: {:#?}",ast);
    let data = match &ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Builder can only be derived for structs"),
    };
    let fields = match &data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => panic!("Builder can only be derived for structs with named fields"),
    };
    let field_names:Vec<syn::Ident> = fields.iter().map(|f| f.ident.clone()).filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<_>>();
    let field_types :Vec<syn::Type> = fields.iter().map(|f| f.ty.clone()).collect::<Vec<_>>();


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
        impl #bident {
            #(
                pub fn #field_names(&mut self, #field_names: #field_types) -> &mut Self {
                    self.#field_names = Some(#field_names);
                    self
                }

            )*
            pub fn build(&self)-> Option<#name> {
                Some(
                #name {
                    #(
                        #field_names: self.#field_names.clone().expect(&format!("{} is not set", stringify!(#field_names))),
                    )*
                }
            )
            }
        }

    };
    TokenStream::from(expanded)
}
