use func::*;
use number::Number;
use std::fmt::Debug;

pub mod compiler;
pub mod env;
pub mod expr;
mod func;
pub mod number;
mod parser;
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
    fn flatten(self) -> Result<Value<T, N>, EvalError> {
        use Value::*;
        Ok(match self {
            Constant(_) | Variable(_) => self,
            FunctionInvoke(crate::func::FunctionInvoke { func, args }) => {
                FunctionInvoke(crate::func::FunctionInvoke {
                    func,
                    args: args
                        .into_iter()
                        .map(|a| a.flatten())
                        .collect::<Result<Vec<Value<T, N>>, EvalError>>()?,
                })
            }
            BinaryOperation(op, mut args) => {
                let [left, right] = *args;
                *args = [left.flatten()?, right.flatten()?];
                if let [Value::Constant(a), Value::Constant(b)] = &*args {
                    Value::Constant(op(*a, *b)?)
                } else {
                    BinaryOperation(op, args)
                }
            }
            UnaryOperation(op, mut arg) => {
                *arg = (*arg).flatten()?;
                if let Value::Constant(val) = &*arg {
                    Value::Constant(op(*val)?)
                } else {
                    UnaryOperation(op, arg)
                }
            }
        })
    }
}

impl<T: Number, const N: usize> Value<T, N> {
    fn evaluate(&self, params: &[T; N]) -> Result<T, EvalError> {
        match self {
            Self::Constant(val) => Ok(*val),
            Self::Variable(ind) => Ok(params[*ind]),
            Self::BinaryOperation(op, args) => {
                op(args[0].evaluate(params)?, args[1].evaluate(params)?)
            }
            Self::UnaryOperation(op, arg) => op(arg.evaluate(params)?),
            Self::FunctionInvoke(func) => func.invoke(params),
        }
    }
}
