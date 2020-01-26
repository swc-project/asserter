# asserter

[![Build Status](https://travis-ci.com/swc-project/asserter.svg?branch=master)](https://travis-ci.com/swc-project/asserter)

A new testing utility for rust

# Features
 - Works on stable
 

 - `unwrap!`
 
You can easily unwrap nested value easily.

```rust
use asserter::*;

enum Complex {
    Normal(String),
    Boxed(Box<Complex>),
}

#[asserter]
fn main() {
    let foo = Complex::Boxed(Box::new(Complex::Normal(String::from("foo"))));

    unwrap!(foo as Complex::Boxed(unbox!(Complex::Normal(s))));
    assert_eq!(s, "foo");   
}
```

Also, you can use box patterns in  `unwrap!`.

```rust
use asserter::*;

enum Complex {
    Normal(String),
    Boxed(Box<Complex>),
}

#[asserter]
fn main() {
    let foo = Complex::Boxed(Box::new(Complex::Normal(String::from("foo"))));

    unwrap!(foo as Complex::Boxed(box Complex::Normal(s)));
    assert_eq!(s, "foo");   
}
```

 - rustfmt-friendly

There are some syntax sugars to allow using rustfmt with it.


# Usage

`Cargo.tml`:
```toml
[dependencies]
asserter = "0.1"
```

```rust
#[asserter]
fn main() {
    let foo = Complex::Boxed(Box::new(Complex::Normal(String::from("foo"))));

    unwrap!(foo as Complex::Boxed(box Complex::Normal(s)));
    assert_eq!(s, "foo");   
}
```