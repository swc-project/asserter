use pmutil::{q, ToTokensExt};
use proc_macro2::{Ident, Span, TokenStream};
use syn::{
    fold::{fold_pat, Fold},
    parse,
    parse::{Parse, ParseStream},
    parse2, Block, Expr, ExprIf, ExprLet, ExprPath, ExprUnary, LitStr, Pat, PatIdent, Stmt, Token,
    UnOp,
};

pub fn expand(input: TokenStream, cons: Vec<Stmt>) -> Expr {
    let mut input: Input = parse2(input).expect("unwrap!(): failed to parse input");

    input.pat = Expander.fold_pat(input.pat);

    expand_to_if_let(Expr::Path(input.expr), input.pat, cons)
}

fn expand_to_if_let(expr: Expr, mut pat: Pat, cons: Vec<Stmt>) -> Expr {
    let else_branch = q!(
        Vars {
            s: format!("failed to unwrap `{}` as `{}`", expr.dump(), pat.dump())
        },
        { panic!(s) }
    )
    .parse();

    if let Pat::Tuple(ref p) = pat {
        if p.elems.len() == 1 {
            return expand_to_if_let(expr, p.elems.first().unwrap().clone(), cons);
        }
    }

    if match pat {
        Pat::Ident(..)
        | Pat::Path(..)
        | Pat::Lit(..)
        | Pat::Tuple(..)
        | Pat::Macro(..)
        | Pat::Struct(..) => true,

        Pat::TupleStruct(ref p) => {
            // If all element is simple, preserve it.
            p.pat.elems.iter().all(|p| match p {
                Pat::Ident(..) | Pat::Lit(_) => true,
                _ => false,
            })
        }

        // Temporarily true
        Pat::Or(_) | Pat::Range(_) | Pat::Rest(_) | Pat::Slice(_) | Pat::Type(_) => true,
        _ => false,
    } {
        // We cannot expand it more.

        let let_expr = Expr::Let(ExprLet {
            attrs: vec![],
            let_token: Default::default(),
            pat,
            eq_token: Default::default(),
            expr: Box::new(expr),
        });

        return Expr::If(ExprIf {
            attrs: vec![],
            if_token: Default::default(),
            cond: Box::new(let_expr),
            then_branch: Block {
                brace_token: Default::default(),
                stmts: cons,
            },
            else_branch: Some((Default::default(), else_branch)),
        });
    }

    match pat {
        // TODO: Remove length restriction. This can be done by using tuples.
        Pat::TupleStruct(ref mut p) if p.pat.elems.len() == 1 => {
            let tmp_ident = Ident::new("_tmp", Span::call_site());
            let tmp_expr = Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: tmp_ident.clone().into(),
            });

            //
            let stmt = Stmt::Semi(
                expand_to_if_let(tmp_expr, Pat::Tuple(p.pat.clone()), cons),
                Default::default(),
            );
            for mut p in p.pat.elems.pairs_mut() {
                let v = p.value_mut();
                **v = Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: tmp_ident.clone(),
                    subpat: None,
                });
            }

            return expand_to_if_let(expr, pat, vec![stmt]);
        }

        Pat::Box(p) => {
            return expand_to_if_let(
                Expr::Unary(ExprUnary {
                    attrs: vec![],
                    op: UnOp::Deref(Default::default()),
                    expr: Box::new(expr),
                }),
                *p.pat.clone(),
                cons,
            );
        }

        Pat::Reference(p) => {
            return expand_to_if_let(expr, *p.pat.clone(), cons);
        }

        Pat::Wild(_) => panic!("uwnrap() does not accept wildcard pattern"),

        _ => unimplemented!("Pat: {:?}", pat),
    }
}

struct Input {
    expr: ExprPath,
    _as_token: Option<Token![as]>,
    _first_comma_token: Option<Token![,]>,
    pat: Pat,
    _comma_token: Option<Token![,]>,
    _msg: Option<LitStr>,
    _args: TokenStream,
}

impl Parse for Input {
    fn parse(i: ParseStream) -> parse::Result<Self> {
        Ok(Input {
            expr: i.parse()?,
            _as_token: i.parse()?,
            _first_comma_token: i.parse()?,
            pat: i.parse()?,
            _comma_token: i.parse()?,
            _msg: i.parse()?,
            _args: i.parse()?,
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
