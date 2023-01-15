use crate::{
    compiler::ExpressionCompiler, env::ExprEnv, parser::ParserError, EvalError, Number, Value,
};

#[derive(Debug, Clone)]
/// A compiled expression which evaluates to the numeric type T and requires N variable values to evaluate
pub struct Expr<T: Number, const N: usize>(Value<T, N>);

impl<T: Number, const N: usize> Expr<T, N> {
    /// Compile an expression from a string-convertible type.
    /// Example:
    /// ```
    /// use crunch_eval::expr::Expr;
    ///
    /// let expr = "1 + 1";
    /// let expr = Expr::compile_env(expr, Default::default()).unwrap();
    /// let value: f32 = expr.evaluate(&[]).unwrap();
    /// ```
    pub fn compile_env(s: impl Into<String>, env: ExprEnv<T, N>) -> Result<Expr<T, N>, ParserError> {
        ExpressionCompiler::compile(s, env).map(|e| Expr(e))
    }

    /// Evaluate the expression by supplying its variable values
    pub fn evaluate(&self, vars: &[T; N]) -> Result<T, EvalError> {
        self.0.evaluate(vars)
    }

    /// Evaluate by passing 0 for all variable values
    pub fn evaluate_zero(&self) -> Result<T, EvalError> {
        self.evaluate(&[Default::default(); N])
    }

    /// Inline operations on constant values to speed up evaluation
    pub fn flatten(self) -> Result<Expr<T, N>, EvalError> {
        Ok(Expr(self.0.flatten()?))
    }
}

impl<T: Number> Expr<T, 0> {
    /// Compile an expression with a blank (default) environment
    pub fn compile(s: impl Into<String>) -> Result<Expr<T, 0>, ParserError> {
        Self::compile_env(s, Default::default())
    }

    /// Evaluate by passing no variable values
    pub fn evaluate_blank(&self) -> Result<T, EvalError> {
        self.evaluate(&[])
    }
}
