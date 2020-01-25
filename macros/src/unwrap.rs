use pmutil::{q, Quote, ToTokensExt};
use proc_macro2::{Ident, TokenStream};
use syn::{
    fold::{fold_block, Fold},
    parse,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Token,
    visit::Visit,
    Block, Expr, ExprAssign, ExprBlock, ExprIf, ExprLet, ExprPath, LitStr, Macro, Pat, PatIdent,
    PatTuple, Stmt, Token,
};

pub fn expand(input: TokenStream) -> Expr {
    let input: Input = parse2(input).expect("unwrap!(): failed to parse input");
    let idents = {
        let mut v = PatIdentCollector::default();
        v.visit_pat(&input.pat);
        v.ident
    };

    let let_expr = Expr::Let(ExprLet {
        attrs: vec![],
        let_token: Default::default(),
        pat: Pat::Tuple(PatTuple {
            attrs: vec![],
            paren_token: Default::default(),
            elems: idents,
        }),
        eq_token: Default::default(),
        expr: Box::new(Expr::Path(input.expr)),
    });

    let_expr
}

#[derive(Default)]
struct PatIdentCollector {
    ident: Punctuated<Pat, Token![,]>,
}

impl Visit<'_> for PatIdentCollector {
    fn visit_pat_ident(&mut self, i: &PatIdent) {
        self.ident.push(Pat::Ident(i.clone()));
    }
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
