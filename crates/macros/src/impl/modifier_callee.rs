use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn};

/// 対象: 関数
/// 動作: 文脈への制約を追加
pub fn proc_macro_impl(args: TokenStream, ast: ItemFn) -> TokenStream {
    let target_trait = args;

    let fn_visibility = ast.vis;
    let fn_ident = ast.sig.ident;
    let fn_ret_type = ast.sig.output;
    let fn_body = ast.block;

    let fn_args = ast.sig.inputs;
    let fn_args = if let Some(FnArg::Receiver(..)) = fn_args.first() {
        let receiver = &fn_args[0];
        let remains = fn_args.iter().skip(1).collect::<Vec<_>>();
        quote! { #receiver, ctx: Ctx, #(#remains),*}
    } else {
        quote! { ctx: Ctx, #fn_args }
    };

    quote! {
        // マクロ適用関数
        #fn_visibility fn #fn_ident <Ctx> (#fn_args) #fn_ret_type
        where
            // 制約
            Ctx: #target_trait,
        {
            // 関数本体
            #fn_body
        }
    }
}
