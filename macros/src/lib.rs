extern crate proc_macro;

use pmutil::ToTokensExt;
use syn::{
    fold::{fold_block, fold_expr, fold_pat, Fold},
    parse::{self, Parse, ParseStream},
    parse2, Block, Error, Expr, ExprBlock, ExprIf, ExprLet, ExprMacro, ExprPath, ImplItem, Item,
    Macro, Pat, Stmt, Token,
};

mod unbox;
mod unwrap;

#[proc_macro_attribute]
pub fn asserter(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse2::<Item>(input.clone().into()) {
        Ok(v) => Expander.fold_item(v).dump().into(),
        Err(_) => match parse2::<ImplItem>(input.into()) {
            Ok(v) => Expander.fold_impl_item(v).dump().into(),
            Err(_) => panic!("failed to parse input as Item / ImplItem"),
        },
    }
}

struct Expander;

impl Fold for Expander {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Macro(e) if e.mac.path.is_ident("unwrap") => {
                let e = self::unwrap::expand(e.mac.tokens).parse();
                fold_expr(self, e)
            }

            _ => fold_expr(self, e),
        }
    }

    fn fold_pat(&mut self, p: Pat) -> Pat {
        match p {
            Pat::Macro(p) if p.mac.path.is_ident("unbox") => {
                let p = self::unbox::expand(p.mac.tokens).parse();
                fold_pat(self, p)
            }

            _ => fold_pat(self, p),
        }
    }
}
