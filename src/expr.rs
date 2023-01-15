use crate::{
    compiler::ExpressionCompiler, env::ExprEnv, parser::ParserError, EvalError, Number, Value,
};

#[derive(Debug, Clone)]
pub struct Expr<T: Number, const N: usize>(Value<T, N>);

impl<T: Number, const N: usize> Expr<T, N> {
    pub fn compile(s: impl Into<String>, env: ExprEnv<T, N>) -> Result<Expr<T, N>, ParserError> {
        ExpressionCompiler::compile(s, env).map(|e| Expr(e))
    }

    pub fn evaluate(&self, vars: &[T; N]) -> Result<T, EvalError> {
        self.0.evaluate(vars)
    }

    pub fn flatten(self) -> Result<Expr<T, N>, EvalError> {
        Ok(Expr(self.0.flatten()?))
    }
}
