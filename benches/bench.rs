use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crunch_eval::{env::ExprEnv, expr::Expr};
use evalexpr::*;

fn compile_expr(expr: &str) -> Expr<f32, 0> {
    Expr::compile(expr).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("compile long expression crunch_eval", |b| b.iter(|| compile_expr(black_box("6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4"))));
    c.bench_function("compile long expression evalexpr", |b| b.iter(|| evalexpr::eval("6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4")));
    c.bench_function("evaluate long expression crunch_eval no flatten", |b| {
        let expr = Expr::<f32, 0>::compile("6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4");
        let expr = expr.unwrap();
        b.iter(|| expr.evaluate(&[]).unwrap());
    });
    c.bench_function("evaluate long expression crunch_eval flatten", |b| {
        let expr = Expr::<f32, 0>::compile("6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4");
        let expr = expr.unwrap().flatten().unwrap();
        b.iter(|| expr.evaluate(&[]).unwrap());
    });
    c.bench_function("evaluate long expression evalexpr", |b| {
        let expr = build_operator_tree("6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4");
        let expr = expr.unwrap();
        b.iter(|| expr.eval().unwrap());
    });
    c.bench_function(
        "evaluate expression with function and variable crunch_eval",
        |b| {
            let env = ExprEnv::new(["x"]).with_func("double", |[x]: [f32; 1]| x * 2.0);
            let expr = Expr::compile_env("double(x + 1)", env).unwrap();
            let vars = &[25.0];
            b.iter(|| expr.evaluate(vars).unwrap());
        },
    );
    c.bench_function(
        "evaluate expression with function and variable evalexpr",
        |b| {
            let context = context_map! {
                 "double" => Function::new(|argument| {
                     if let Ok(num) = argument.as_float() {
                         Ok(Value::Float(num * 2.0))
                     } else {
                         Err(EvalexprError::expected_number(argument.clone()))
                     }
                 }),
                 "x" => 24,
            }
            .unwrap();
            let expr = build_operator_tree("double(x+1.0)").unwrap();
            b.iter(|| expr.eval_with_context(&context).unwrap());
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
