use crunch_eval::expr::Expr;

fn main() {
    loop {
        std::io::stdin()
            .lines()
            .into_iter()
            .map(|line| line.unwrap())
            .for_each(|line| {
                let expr = Expr::compile(line).unwrap();
                let val: f64 = expr.evaluate(&[]).unwrap();
                println!(" = {}", val);
            });
    }
}
