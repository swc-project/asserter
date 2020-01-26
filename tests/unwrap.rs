use asserter::*;

#[test]
#[asserter]
fn simple() {
    let foo = Some("f");

    unwrap!(foo as Some(foo));
    assert_eq!(foo, "f");
}

#[test]
#[should_panic]
#[asserter]
fn simple_panic() {
    let foo: Option<String> = None;

    unwrap!(foo as Some(foo));
}

enum Complex {
    Path,
    Normal(String),
    Boxed(Box<Complex>),
    Ref(&'static Complex),
    Tuple(String, usize),
    Struct { foo: usize, bar: usize },
}

#[test]
#[asserter]
fn complex_boxed() {
    let foo = Complex::Boxed(Box::new(Complex::Normal(String::from("foo"))));

    unwrap!(foo as Complex::Boxed(unbox!(Complex::Normal(s))));

    assert_eq!(s, "foo");
}

#[test]
#[asserter]
fn complex_ref() {
    static COMPLEX: Complex = Complex::Path;
    let foo = Complex::Ref(&COMPLEX);

    unwrap!(foo as Complex::Ref(&Complex::Path));
}

#[test]
#[asserter]
fn complex_path() {
    let foo = Complex::Path;

    unwrap!(foo as Complex::Path);
}

#[test]
#[asserter]
fn complex_tuple() {
    let foo = Complex::Tuple(String::new(), 5);

    unwrap!(foo as Complex::Tuple(s, v));
    assert_eq!(s, "");
    assert_eq!(v, 5);
}

#[test]
#[asserter]
fn complex_struct() {
    let foo = Complex::Struct { foo: 1, bar: 2 };

    unwrap!(foo, Complex::Struct { foo, bar });
}
