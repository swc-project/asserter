use pmutil::{q, Quote};
use proc_macro2::TokenStream;
use syn::{parse2, Pat};

pub fn expand(input: TokenStream) -> Quote {
    let input: Pat = parse2(input).expect("unbox!(): Expected expression");

    q!(Vars { input }, (box input))
}
