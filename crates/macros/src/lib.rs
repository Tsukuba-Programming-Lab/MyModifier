mod r#impl;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, ItemFn, ItemTrait};

#[proc_macro_attribute]
pub fn modifier(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: TokenStream = attr.into();
    let ast = parse_macro_input!(item as ItemTrait);
    r#impl::modifier::proc_macro_impl(args, ast).into()
}

#[proc_macro_attribute]
pub fn modifier_caller(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: TokenStream = attr.into();
    let ast = parse_macro_input!(item as ItemFn);
    r#impl::modifier_caller::proc_macro_impl(args, ast).into()
}
