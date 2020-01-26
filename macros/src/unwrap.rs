use pmutil::{q, Quote, ToTokensExt};
use proc_macro2::{Ident, TokenStream};
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

pub fn expand(input: TokenStream) -> Expr {
    let mut input: Input = parse2(input).expect("unwrap!(): failed to parse input");
    input.pat = Expander.fold_pat(input.pat);

    let idents = {
        let mut v = PatIdentCollector::default();
        v.visit_pat(&input.pat);
        v.ident.push_punct(Default::default());
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
        expr: Box::new(expand_match(Expr::Path(input.expr), input.pat)),
    });

    let_expr
}

fn expand_match(expr: Expr, pat: Pat) -> Expr {
    Expr::Match(ExprMatch {
        attrs: vec![],
        match_token: Default::default(),
        expr: Box::new(expr),
        brace_token: Default::default(),
        arms: vec![Arm {
            attrs: vec![],
            pat,
            guard: None,
            fat_arrow_token: Default::default(),
            body: q!(Vars {}, ({})).parse(),
            comma: None,
        }],
    })
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

struct Expander;

impl Fold for Expander {
    fn fold_pat(&mut self, p: Pat) -> Pat {
        match p {
            Pat::Macro(p) if p.mac.path.is_ident("unbox") => {
                let p = crate::unbox::expand(p.mac.tokens).parse();
                fold_pat(self, p)
            }

            _ => fold_pat(self, p),
        }
    }
}
