use proc_macro2::{TokenStream, Ident};
use quote::quote;
use syn::ItemTrait;

/// 対象: トレイト
/// 動作: 文脈導入用の宣言型マクロを生成
pub fn proc_macro_impl(_args: TokenStream, ast: ItemTrait) -> TokenStream {
    let trait_name = &ast.ident;
    let macro_ident = as_macro_ident(trait_name);

    quote! {
        // トレイト本体
        #ast

        // 文脈マクロ
        #[macro_export]
        macro_rules! #macro_ident {
            ({ $($body:tt)* }) => {
                // 文脈を用意
                struct Ctx;

                // 文脈付き呼び出しへの変換 (関数定義ではなくブロックに変換される)
                #[modifier_caller(#trait_name)]
                fn __mymodifier_caller() {
                    $($body)*
                }
            };

            ($($body:tt)*) => {{
                // 文脈付き呼び出しへの変換 (関数定義ではなくブロックに変換される)
                #[modifier_caller(#trait_name)]
                fn __mymodifier_caller() {
                    $($body)*
                }
            }};
        }
    }
}

fn as_macro_ident(ident: &Ident) -> TokenStream {
    let mut result = String::new();
    for c in ident.to_string().chars() {
        if c.is_uppercase() {
            result.push('_');
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    let result = if result.starts_with('_') {
        &result[1..]
    } else {
        &result
    };

    result.parse().unwrap()
}
