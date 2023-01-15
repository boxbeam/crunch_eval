use std::collections::HashMap;

use crate::{
    compiler::Token,
    func::{CustomFunc, Function},
    number::Trig,
    Number, Value,
};

/// An environment specifying variables and functions an expression can use
///
/// Example:
/// ```
/// use crunch_eval::{expr::Expr, env::ExprEnv};
///
/// let env = ExprEnv::new(["x", "y"])
///     .with_func("abs", |[x]: [i32; 1]| x.abs());
/// let expr = Expr::compile_env("x * abs(y)", env).unwrap();
/// let val: i32 = expr.evaluate(&[4, -7]).unwrap();
/// assert_eq!(val, 28);
/// ```
pub struct ExprEnv<T: Number, const N: usize> {
    named_tokens: HashMap<String, Token<T, N>>,
}

impl<T: Number> Default for ExprEnv<T, 0> {
    /// An ExprEnv with no variables and no functions
    fn default() -> Self {
        ExprEnv {
            named_tokens: HashMap::new(),
        }
    }
}

impl<T: Number, const N: usize> ExprEnv<T, N> {
    /// Create a new expression environment by specifying the variable names that will be used
    pub fn new(var_names: [&str; N]) -> ExprEnv<T, N> {
        ExprEnv {
            named_tokens: var_names
                .map(|s| s.to_owned())
                .iter()
                .enumerate()
                .map(|(index, elem)| (elem.clone(), Token::Value(Value::Variable(index))))
                .collect(),
        }
    }

    pub(crate) fn get(&self, name: &str) -> Option<&Token<T, N>> {
        self.named_tokens.get(name)
    }

    /// Add a custom function that can be used during evaluation
    /// Example:
    /// ```
    /// use crunch_eval::{expr::Expr, env::ExprEnv};
    ///
    /// let env = ExprEnv::default().with_func("min", |[a, b]: [f64; 2]| a.min(b))
    ///     .with_func("max", |[a, b]| a.max(b));
    /// let expr = Expr::compile_env("max(1, 2)", env).unwrap();
    /// assert_eq!(expr.evaluate_blank().unwrap(), 2.0);
    /// ```
    pub fn with_func<const A: usize, F: CustomFunc<T, N, A>>(
        mut self,
        name: impl Into<String>,
        func: F,
    ) -> Self {
        let function = Function::new::<A, F>(func);
        self.named_tokens
            .insert(name.into(), Token::Function(function));
        self
    }
}

impl<T: Number + Trig, const N: usize> ExprEnv<T, N> {
    /// Add trig functions (sin, cos, tan)
    pub fn with_trig(self) -> Self {
        self.with_func("sin", |[x]: [T; 1]| x.sin())
            .with_func("cos", |[x]: [T; 1]| x.cos())
            .with_func("tan", |[x]: [T; 1]| x.tan())
    }
}
