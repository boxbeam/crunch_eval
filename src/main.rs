use crunch_eval::expr::Expr;

fn main() {
    let expr = "6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4";
    let expr = Expr::compile(expr, Default::default()).unwrap();
    let val: f64 = expr.evaluate(&[]).unwrap();
    println!("{}", val);
}