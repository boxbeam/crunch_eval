use crate::expr::Expr;

#[test]
fn test() {
    assert_eq!(
        Expr::<i32, 0>::compile("1 + 1", Default::default())
            .unwrap()
            .evaluate(&[])
            .unwrap(),
        2
    );
    Expr::<i32, 0>::compile("1 / 0", Default::default())
        .unwrap()
        .evaluate(&[])
        .expect_err("divide by zero");
    assert_eq!(Expr::<f64, 0>::compile("6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4", Default::default())
            .unwrap()
            .evaluate(&[])
            .unwrap(),
        402193.3186140596f64
    );
}
