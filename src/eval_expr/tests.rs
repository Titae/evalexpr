use super::*;

#[test]
fn basic() {
    assert_eq!(1., evaluate("1+1-1").unwrap(), "1+1-1");
    assert_eq!(4., evaluate("(1+1)*2").unwrap(), "(1+1)*2");
    assert_eq!(-1.25, evaluate("(-(1.2+1.3)*2)/(2.35+1.65)").unwrap(), "(-(1.2+1.3)*2)/(2.35+1.65)");
}

#[test]
fn advanced() {
    assert_eq!(0.1, evaluate("(-(-(-(-(-42^-(42%21*3)+2.01)-12/33.33))*0.001)+0.10)").unwrap(), "(-(-(-(-(-42^-(42%21*3)+2.01)-12/33.33))*0.001)+0.10)");
}

#[test]
fn addition_evaluate() {
    assert_eq!(2., evaluate("1+1").unwrap(), "1+1");
    assert_eq!(0., evaluate("-1+1").unwrap(), "-1+1");
    assert_eq!(0., evaluate("1+-1").unwrap(), "1+-1");
    assert_eq!(2., evaluate("+1+++1").unwrap(), "+1+++1");
}

#[test]
fn substraction_evaluate() {
    assert_eq!(0., evaluate("1-1").unwrap(), "1-1");
    assert_eq!(-2., evaluate("-1-1").unwrap(), "-1-1");
    assert_eq!(2., evaluate("1--1").unwrap(), "1--1");
    assert_eq!(0., evaluate("-1--1").unwrap(), "-1--1");
    assert_eq!(0., evaluate("-1-+---1").unwrap(), "-1-+---1");
}

#[test]
fn division_evaluate() {
    assert_eq!(1., evaluate("1/1").unwrap(), "1/1");
    assert_eq!(0.5, evaluate("1/2").unwrap(), "1/2");
    assert_eq!(0.33, evaluate("1/3").unwrap(), "1/3");
    assert_eq!(0.67, evaluate("2/3").unwrap(), "2/3");
    assert_eq!(-1., evaluate("-1/1").unwrap(), "-1/1");
    assert_eq!(-1., evaluate("1/-1").unwrap(), "1/-1");
    assert_eq!(1., evaluate("-1/-1").unwrap(), "-1/-1");
}

#[test]
fn multiplication_evaluate() {
    assert_eq!(1., evaluate("1*1").unwrap(), "1*1");
    assert_eq!(2., evaluate("1*2").unwrap(), "1*2");
    assert_eq!(-2., evaluate("-1*2").unwrap(), "-1*2");
    assert_eq!(2., evaluate("-1*-2").unwrap(), "-1*-2");
}

#[test]
fn modulo_evaluate() {
    assert_eq!(0., evaluate("1%1").unwrap(), "1%1");
    assert_eq!(1., evaluate("1%2").unwrap(), "1%2");
    assert_eq!(0., evaluate("2%1").unwrap(), "2%1");
    assert_eq!(0., evaluate("-2%1").unwrap(), "-2%1");
    assert_eq!(0., evaluate("-2%-1").unwrap(), "-2%-1");
}

#[test]
fn pow_evaluate() {
    assert_eq!(1., evaluate("1^0").unwrap(), "1^0");
    assert_eq!(1., evaluate("1^1").unwrap(), "1^1");
    assert_eq!(2., evaluate("2^1").unwrap(), "2^1");
    assert_eq!(4., evaluate("2^2").unwrap(), "2^2");
    assert_eq!(-4., evaluate("-2^2").unwrap(), "-2^2");
    assert_eq!(-0.25, evaluate("-2^-2").unwrap(), "-2^-2");
}
