use pmutil::{q, Quote, ToTokensExt};
use proc_macro2::{Ident, Span, TokenStream};
use syn::{
    fold::{fold_block, fold_pat, Fold},
    parse,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Token,
    visit::Visit,
    Arm, Block, Expr, ExprAssign, ExprBlock, ExprIf, ExprLet, ExprMatch, ExprPath, LitStr, Macro,
    Pat, PatIdent, PatMacro, PatTuple, Stmt, Token,
};

pub fn expand(input: TokenStream, cons: Vec<Stmt>) -> Expr {
    let mut input: Input = parse2(input).expect("unwrap!(): failed to parse input");

    let else_branch = q!(
        Vars {
            s: format!(
                "failed to unwrap `{}` as `{}`",
                input.expr.dump(),
                input.pat.dump()
            )
        },
        { panic!(s) }
    )
    .parse();

    input.pat = Expander.fold_pat(input.pat);

    let let_expr = Expr::Let(ExprLet {
        attrs: vec![],
        let_token: Default::default(),
        pat: input.pat.clone(),
        eq_token: Default::default(),
        expr: Box::new(Expr::Path(input.expr)),
    });

    Expr::If(ExprIf {
        attrs: vec![],
        if_token: Default::default(),
        cond: Box::new(let_expr),
        then_branch: Block {
            brace_token: Default::default(),
            stmts: cons,
        },
        else_branch: Some((Default::default(), else_branch)),
    })
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

struct Expander;

impl Fold for Expander {
    fn fold_pat(&mut self, p: Pat) -> Pat {
        match p {
            Pat::Macro(p) if p.mac.path.is_ident("unbox") => {
                let p = crate::unbox::expand(p.mac.tokens).parse();
                fold_pat(self, p)
            }

            // TODO: use ref
            // Pat::Reference(p) => *p.pat,
            _ => fold_pat(self, p),
        }
    }
}
