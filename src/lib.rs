use std::str::FromStr;
use num::{Num, CheckedAdd, CheckedSub, CheckedDiv, CheckedMul, traits::CheckedRem};
use func::*;

mod parser;
pub mod compiler;
pub mod env;
mod func;
mod linked_list;

pub trait Number: 'static + Num + FromStr + Copy + Default + Pow<Self> + CheckedAdd + CheckedSub + CheckedDiv + CheckedMul + CheckedRem {}
impl<T: 'static + Num + FromStr + Copy + Default + Pow<Self> + CheckedAdd + CheckedSub + CheckedDiv + CheckedMul + CheckedRem> Number for T {}

#[derive(Debug)]
pub enum EvalError {
    NegativeIntegerExponent,
    DivideByZero,
    Overflow,
}

pub trait Pow<Rhs> {
    fn pow(&self, exp: Rhs) -> Result<Rhs, EvalError>;
}

macro_rules! impl_pow {
    ($lhs:ty, $rhs:ty) => {
        #[allow(unused_comparisons)]
        impl Pow<Self> for $lhs {
            fn pow(&self, exp: Self) -> Result<Self, EvalError> {
                if exp < 0 {
                    return Err(EvalError::NegativeIntegerExponent);
                }
                Ok(<$lhs>::pow(*self, exp as $rhs))
            }
        }
    };
    ($lhs: ty, $rhs: ty, f) => {
        impl Pow<Self> for $lhs {
            fn pow(&self, exp: Self) -> Result<Self, EvalError> {
                Ok(<$lhs>::powf(*self, exp as $rhs))
            }
        }
    }
}

impl_pow!(i32, u32);
impl_pow!(u32, u32);
impl_pow!(i8, u32);
impl_pow!(u8, u32);
impl_pow!(i16, u32);
impl_pow!(u16, u32);
impl_pow!(i64, u32);
impl_pow!(u64, u32);
impl_pow!(i128, u32);
impl_pow!(f32, f32, f);
impl_pow!(f64, f64, f);

#[derive(Clone)]
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