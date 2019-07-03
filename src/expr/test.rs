use std::fmt::Display;

use super::*;
use super::tags::*;

use crate::Hamacher1;


#[test]
fn basic() {
    let a = Expr::<Hamacher1>::new(0.1);
    let b = Expr::new(0.6);
    let c = Expr::new(0.4);
    let d = a.or(b).and(c.not());

    let string = d.to_string();
    let value = d.to_value().as_raw();

    assert_eq!(string, "((0.1 | 0.6) & !0.4)");
    assert!((value - 0.384).abs() < 0.0001);
}

#[test]
fn ops() {
    let a = Expr::<Hamacher1>::new(0.1);
    let b = Expr::new(0.6);
    let c = Expr::new(0.4);
    let d = (a | b) & !c;

    let string = d.to_string();
    let value = d.to_value().as_raw();

    assert_eq!(string, "((0.1 | 0.6) & !0.4)");
    assert!((value - 0.384).abs() < 0.0001);
}

#[test]
fn named() {
    let a = Expr::<Hamacher1>::new(0.1).with_tag("a");
    let b = Expr::new(0.6).with_tag("b");
    let c = Expr::new(0.4).with_tag("c");
    let d = a.or(b).and(c.not());

    let string = d.to_string();
    let value = d.to_value().as_raw();

    assert_eq!(string, "((a | b) & !c)");
    assert!((value - 0.384).abs() < 0.0001);
}

#[test]
fn order_small() {
    // erasure:
    type H = Hamacher1;
    trait ExprStr = Expr<H> + Display + Copy;
    fn abc() -> (impl ExprStr, impl ExprStr, impl ExprStr) {
        let a = Expr::new(0.1).with_tag(TagA);
        let b = Expr::new(0.2).with_tag(TagB);
        let c = Expr::new(0.3).with_tag(TagC);
        (a, b, c)
    }
    let (a, b, c) = abc();


    // test:
    let d = fuzzy_math!(a & b | c);
    assert_eq!(d.to_string(), "((a & b) | c)");

    let d = fuzzy_math!(!a & b | c);
    assert_eq!(d.to_string(), "((!a & b) | c)");

    let d = fuzzy_math!(a & !b | c);
    assert_eq!(d.to_string(), "((a & !b) | c)");

    let d = fuzzy_math!(a & b | !c);
    assert_eq!(d.to_string(), "((a & b) | !c)");


    let d = fuzzy_math!(a | b & c);
    assert_eq!(d.to_string(), "(a | (b & c))");

    let d = fuzzy_math!(!a | b & c);
    assert_eq!(d.to_string(), "(!a | (b & c))");

    let d = fuzzy_math!(a | !b & c);
    assert_eq!(d.to_string(), "(a | (!b & c))");

    let d = fuzzy_math!(a | b & !c);
    assert_eq!(d.to_string(), "(a | (b & !c))");
}

#[test]
fn order_big() {
    // erasure:
    type H = Hamacher1;
    trait ExprStr = Expr<H> + Display + Copy;
    fn abcd() -> (impl ExprStr, impl ExprStr, impl ExprStr, impl ExprStr) {
        let a = Expr::new(0.1).with_tag(TagA);
        let b = Expr::new(0.2).with_tag(TagB);
        let c = Expr::new(0.3).with_tag(TagC);
        let d = Expr::new(0.4).with_tag(TagD);
        (a, b, c, d)
    }
    let (a, b, c, d) = abcd();

    // test:
    let e = fuzzy_math!(a | b | c | d);
    assert_eq!(e.to_string(), "(((a | b) | c) | d)");

    let e = fuzzy_math!(a | b & c | d);
    assert_eq!(e.to_string(), "((a | (b & c)) | d)");

    let e = fuzzy_math!(a | !b & c | d);
    assert_eq!(e.to_string(), "((a | (!b & c)) | d)");

    let e = fuzzy_math!(a | b & !c | d);
    assert_eq!(e.to_string(), "((a | (b & !c)) | d)");

    let e = fuzzy_math!(a | b & c | !d);
    assert_eq!(e.to_string(), "((a | (b & c)) | !d)");


    let e = fuzzy_math!((a | b) & c | d);
    assert_eq!(e.to_string(), "(((a | b) & c) | d)");

    let e = fuzzy_math!(a | b & (c | d));
    assert_eq!(e.to_string(), "(a | (b & (c | d)))");

    let e = fuzzy_math!((a | b) & (c | d));
    assert_eq!(e.to_string(), "((a | b) & (c | d))");


    let e = fuzzy_math!(a | b & c & d);
    assert_eq!(e.to_string(), "(a | ((b & c) & d))");

    let e = fuzzy_math!(a | !b & c & d);
    assert_eq!(e.to_string(), "(a | ((!b & c) & d))");

    let e = fuzzy_math!(a | b & !c & d);
    assert_eq!(e.to_string(), "(a | ((b & !c) & d))");

    let e = fuzzy_math!(a | b & c & !d);
    assert_eq!(e.to_string(), "(a | ((b & c) & !d))");
}

#[test]
fn zero_cost_tag() {
    let a = Expr::<Hamacher1>::new(0.1);
    let a_tagged = a.clone().with_tag(TagA);
    let a_named = a.clone().with_tag("a");

    fn size_of<T>(_val: T) -> usize {
        std::mem::size_of::<T>()
    }
    assert_eq!(size_of(a), size_of(a_tagged));
    assert_ne!(size_of(a), size_of(a_named));
}
