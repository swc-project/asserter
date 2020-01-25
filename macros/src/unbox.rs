use pmutil::{q, Quote, ToTokensExt};
use proc_macro2::TokenStream;
use syn::{
    fold::{fold_block, Fold},
    parse,
    parse::{Parse, ParseStream},
    parse2, Block, Expr, ExprBlock, ExprIf, ExprLet, ExprPath, LitStr, Macro, Pat, Stmt, Token,
};

pub fn expand(input: TokenStream) -> Quote {
    let input: Pat = parse2(input).expect("unbox!(): Expected expression");

    q!(Vars { input }, (box input))
}
