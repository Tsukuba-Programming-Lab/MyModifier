use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, ItemFn, Stmt};

/// 対象: 関数
/// 動作: 文脈付き呼び出しを行うための前準備を生成 & 文脈付き呼び出しへの変換
pub fn proc_macro_impl(args: TokenStream, ast: ItemFn) -> TokenStream {
    let target_trait = args;

    let stmts = &ast
        .block
        .stmts
        .iter()
        .map(append_ctx_if_calling)
        .collect::<Vec<_>>();

    quote! {
        // 前準備
        #[allow(non_local_definitions)]
        impl #target_trait for Ctx {}

        // 関数本体
        #(#stmts);*
    }
}

fn append_ctx_if_calling(stmt: &Stmt) -> TokenStream {
    let append_ctx = |expr: &Expr| {
        if let Expr::Call(call) = expr {
            let func = call.func.to_token_stream();
            let args = call.args.to_token_stream();
            quote! { #func (Ctx, #args) }
        } else if let Expr::MethodCall(call) = expr {
            let receiver = call.receiver.to_token_stream();
            let method = call.method.to_token_stream();
            let args = call.args.to_token_stream();
            quote! { #receiver . #method (Ctx, #args) }
        } else {
            stmt.to_token_stream()
        }
    };

    match stmt {
        Stmt::Expr(expr, _) => append_ctx(expr),
        _ => stmt.to_token_stream(),
    }
}
