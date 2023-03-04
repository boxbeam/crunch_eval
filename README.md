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

Expression: `6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4 + 6.5*7.8^2.3 + (3.5^3+7/2)^3 -(5*4/(2-3))*4`

Time to compile long expression:

| compile     | time/op (µs) |
|-------------|--------------|
| crunch_eval | 13.56        |
| evalexpr    | 32.36        |

Time to evaluate long expression:

| eval        | time/op (ns) |
|-------------|--------------|
| crunch_eval | 2.97         |
| evalexpr    | 5.96         |

Expression: `double(x + 1)`

Time to evaluate expression with variable and function:

| eval        | time/op (ns) |
|-------------|--------------|
| crunch_eval | 48.46        |
| evalexpr    | 341.83       |
