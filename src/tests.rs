use crate::{expr::Expr, number::Number};

fn should_equal<T: Number + PartialEq>(expr: &str, val: T) {
    assert_eq!(Expr::<T, 0>::compile(expr).unwrap().evaluate_blank().unwrap(), val);
}

#[test]
fn test() {
    should_equal("1 + 1", 2);
    Expr::<i32, 0>::compile("1 / 0").unwrap().evaluate_blank().expect_err("divide by zero");
    should_equal("6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4", 402193.3186140596f64);
}
