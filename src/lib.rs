use std::fmt::Debug;
use func::*;
use number::Number;

pub mod compiler;
pub mod env;
pub mod expr;
pub mod number;
mod parser;
mod func;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum EvalError {
    NegativeIntegerExponent,
    DivideByZero,
    Overflow,
}

#[derive(Clone, Debug)]
enum Value<T: Number, const N: usize> {
    Constant(T),
    Variable(usize),
    BinaryOperation(fn(T, T) -> Result<T, EvalError>, Box<[Value<T, N>; 2]>),
    UnaryOperation(fn(T) -> Result<T, EvalError>, Box<Value<T, N>>),
    FunctionInvoke(FunctionInvoke<T, N>),
}

impl<T: Number, const N: usize> Value<T, N> {
    fn evaluate(&self, params: &[T; N]) -> Result<T, EvalError> {
        match self {
            Self::Constant(val) => Ok(*val),
            Self::Variable(ind) => Ok(params[*ind]),
            Self::BinaryOperation(op, args) => op(args[0].evaluate(params)?, args[1].evaluate(params)?),
            Self::UnaryOperation(op, arg) => op(arg.evaluate(params)?),
            Self::FunctionInvoke(func) => func.invoke(params),
        }
    }
}