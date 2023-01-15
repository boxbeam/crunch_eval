use crunch_eval::{env::ExprEnv, expr::Expr};

fn main() {
    loop {
        std::io::stdin()
            .lines()
            .into_iter()
            .map(|line| line.unwrap())
            .for_each(|line| {
                let expr = Expr::compile(line, ExprEnv::default().with_trig()).unwrap();
                let val: f64 = expr.evaluate(&[]).unwrap();
                println!(" = {}", val);
            });
    }
}
