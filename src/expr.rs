use crate::{Value, Number, parser::ParserError, compiler::ExpressionCompiler, env::ExprEnv, EvalError};

pub struct Expr<T: Number, const N: usize>(Value<T, N>);

impl<T: Number, const N: usize> Expr<T, N> {
    pub fn compile(s: impl Into<String>, env: ExprEnv<T, N>) -> Result<Expr<T, N>, ParserError> {
        ExpressionCompiler::compile(s, env).map(|e| Expr(e))
    }

    pub fn evaluate(&self, vars: &[T; N]) -> Result<T, EvalError> {
        self.0.evaluate(vars)
    }
}