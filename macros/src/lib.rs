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
    fn fold_block(&mut self, b: Block) -> Block {
        let b = fold_block(self, b);
        let mut stmts = vec![];

        let mut base = b.stmts.into_iter();
        loop {
            let stmt = match base.next() {
                Some(v) => v,
                None => break,
            };

            match stmt {
                Stmt::Semi(Expr::Macro(mac), semi) if mac.mac.path.is_ident("unwrap") => {
                    let stmt =
                        Stmt::Semi(crate::unwrap::expand(mac.mac.tokens, base.collect()), semi);
                    stmts.push(stmt);
                    break;
                }
                Stmt::Expr(Expr::Macro(mac)) if mac.mac.path.is_ident("unwrap") => {
                    let stmt = Stmt::Expr(crate::unwrap::expand(mac.mac.tokens, base.collect()));
                    stmts.push(stmt);
                    break;
                }

                _ => stmts.push(stmt),
            }
        }

        Block { stmts, ..b }
    }
}
