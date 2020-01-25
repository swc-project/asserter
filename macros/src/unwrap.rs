use pmutil::{q, Quote, ToTokensExt};
use proc_macro2::TokenStream;
use syn::{
    fold::{fold_block, Fold},
    parse,
    parse::{Parse, ParseStream},
    parse2, Block, Expr, ExprBlock, ExprIf, ExprLet, ExprPath, LitStr, Macro, Pat, Stmt, Token,
};

pub fn expand(input: TokenStream) -> Quote {
    let input: Input = parse2(input).expect("unpack!(): failed to parse input");

    let let_expr = Expr::Let(ExprLet {
        attrs: Default::default(),
        let_token: Default::default(),
        pat: input.pat,
        eq_token: Default::default(),
        expr: Box::new(Expr::Path(input.expr)),
    });

    let if_let_expr = Expr::If(ExprIf {
        attrs: Default::default(),
        if_token: Default::default(),
        cond: Box::new(let_expr),
        then_branch: Block {
            stmts: vec![],
            brace_token: Default::default(),
        },
        else_branch: Some((Default::default(), q!(Vars {}, ({ panic!() })).parse())),
    });

    q!(Vars { if_let_expr }, ({ if_let_expr })).into()
}

struct Input {
    expr: ExprPath,
    _as_token: Token![as],
    pat: Pat,
    _comma_token: Option<Token![,]>,
    msg: Option<LitStr>,
    args: TokenStream,
}

impl Parse for Input {
    fn parse(i: ParseStream) -> parse::Result<Self> {
        Ok(Input {
            expr: i.parse()?,
            _as_token: i.parse()?,
            pat: i.parse()?,
            _comma_token: i.parse()?,
            msg: i.parse()?,
            args: i.parse()?,
        })
    }
}
