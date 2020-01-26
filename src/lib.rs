//! Assertion library for rust.
//!
//! In a function annotated wit assterer, you can use some macros.
//!
//! # `unwrap!(foo as Pat)`
//!
//! ```rust
//! use asserter::*;
//!
//! fn main () {
//!     let foo = Some("example");
//!
//!     // If there's no issue with rustfmt, use `as` like the code below.
//!     unwrap!(foo as Some(s));
//!     assert_eq!(s, "example");
//! }
//! ```
//!
//! Sometimes, rustfmt does not work for the default syntax. There's an
//! alternative syntax to help rustfmt.
//!
//! ```rust
//! use asserter::*;
//! enum Complex {
//!     Struct { foo: usize, bar: usize },
//! }
//! fn main () {
//!     let foo = Complex::Struct { foo: 1, bar: 2 };
//!
//!     // This syntax allows formatting
//!     unwrap!(foo, Complex::Struct { foo, bar });
//! }
//! ```
//!
//! Note that you **can** use box_patterns inside `unwrap!()` (on stable).
//!
//! ```rust
//! enum Complex {
//!     Normal(usize),
//!     Boxed(Box<Complex>),
//! }
//! fn main () {
//!     let foo = Complex::Boxed(Box::new(Complex::Normal(0)));
//!
//!     unwrap!(foo as box Complex::Boxed(Complex::Normal(v)));
//!     assert_eq!(v, 0);
//! }
//! ```
//!
//!
//! ## unbox!(Pat)
//!
//! If you want rustfmt to work correctly while unwrapping box, there's a helper
//! for it.
//!
//! ```rust
//! enum Complex {
//!     Normal(usize),
//!     Boxed(Box<Complex>),
//! }
//! fn main () {
//!     let foo = Complex::Boxed(Box::new(Complex::Normal(0)));
//!
//!     unwrap!(foo as unbox!(Complex::Boxed(Complex::Normal(v))));
//!     assert_eq!(v, 0);
//! }
//! ```
//!  

pub use asserter_macros::asserter;
