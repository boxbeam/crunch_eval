# crunch_eval
High-performance algebraic expression evaluator.

## Basic usage:
```
use crunch_eval::expr::Expr;
let expr = Expr::compile("1 + 1").unwrap();
let val: f64 = expr.evaluate_blank().unwrap();
assert_eq!(val, 2.0);
```

## Benchmarks:
