use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

pub fn proc_macro_impl(_args: TokenStream, ast: ItemTrait) -> TokenStream {
    quote! {
        #ast

        macro_rules! hello {
            () => {
                println!("Hello!");
            };
        }
    }
}
