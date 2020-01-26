use asserter::*;
//
//#[test]
//#[asserter]
//fn simple() {
//    let foo = Some("f");
//
//    unwrap!(foo as Some(foo));
//    assert_eq!(foo, "f");
//}
//
//#[test]
//#[should_panic]
//#[asserter]
//fn simple_panic() {
//    let foo: Option<String> = None;
//
//    unwrap!(foo as Some(foo));
//}

enum Complex {
    Path,
    Normal(String),
    Boxed(Box<Complex>),
    Ref(&'static Complex),
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
    let foo = Complex::Ref(&Complex::Normal(String::from("foo")));

    unwrap!(foo as Complex::Ref(&Complex::Normal(s)));

    assert_eq!(s, "foo");
}

#[test]
#[asserter]
fn complex_path() {
    let foo = Complex::Path;

    unwrap!(foo as Complex::Path);
}
