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
    Normal(String),
    Boxed(Box<Complex>),
}

#[test]
#[asserter]
fn complex() {
    let foo = Complex::Boxed(Box::new(Complex::Normal(String::from("foo"))));

    unwrap!(foo as Complex::Boxed(unbox!(Complex::Normal(s))));

    assert_eq!(s, "foo");
}
