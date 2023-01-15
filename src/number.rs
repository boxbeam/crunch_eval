use std::{str::FromStr, fmt::Debug};

use num::{CheckedAdd, CheckedSub, CheckedDiv, CheckedMul, traits::CheckedRem};

use crate::EvalError;

pub trait NumberOps: Add + Sub + Mul + Div + Rem + Neg + Pow<Self> {}
impl<T: Add + Sub + Mul + Div + Rem + Neg + Pow<Self>> NumberOps for T {}

pub trait Number: 'static + FromStr + Copy + Default + NumberOps + Debug {}
impl<T: 'static + FromStr + Copy + Default + NumberOps + Debug> Number for T {}

macro_rules! op_trait {
    ($name:ident, $op_name:ident) => {
        pub trait $name: Sized {
            fn $op_name(&self, other: Self) -> Option<Self>;
        }
    }
}

op_trait!(Add, add);
op_trait!(Sub, sub);
op_trait!(Mul, mul);
op_trait!(Div, div);
op_trait!(Rem, rem);

macro_rules! impl_op_trait_f {
    ($type:ty, $trait:ty, $name:ident, $op:tt) => {
        impl $trait for $type {
            fn $name(&self, other: Self) -> Option<Self> {
                Some(*self $op other)
            }
        }
    }
}

macro_rules! impl_op_traits_f {
    ($type:ty) => {
        impl_op_trait_f!($type, Add, add, +);
        impl_op_trait_f!($type, Sub, sub, -);
        impl_op_trait_f!($type, Div, div, /);
        impl_op_trait_f!($type, Mul, mul, *);
        impl_op_trait_f!($type, Rem, rem, %);
    }
}

macro_rules! impl_op_trait {
    ($type:ty, $trait:ty, $name:ident, $checked:ident) => {
        impl $trait for $type {
            fn $name(&self, other: Self) -> Option<Self> {
                self.$checked(&other)
            }
        }
    }
}

macro_rules! impl_op_traits {
    ($type:ty) => {
        impl_op_trait!($type, Add, add, checked_add);
        impl_op_trait!($type, Sub, sub, checked_sub);
        impl_op_trait!($type, Mul, mul, checked_mul);
        impl_op_trait!($type, Div, div, checked_div);
        impl_op_trait!($type, Rem, rem, checked_rem);
    }
}

impl_op_traits!(i8);
impl_op_traits!(i16);
impl_op_traits!(i32);
impl_op_traits!(i64);
impl_op_traits!(i128);
impl_op_traits!(u8);
impl_op_traits!(u16);
impl_op_traits!(u32);
impl_op_traits!(u64);
impl_op_traits!(u128);
impl_op_traits_f!(f32);
impl_op_traits_f!(f64);

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

pub trait Neg {
    fn neg(&self) -> Self;
}

impl<T: Sub + Default + Copy> Neg for T {
    fn neg(&self) -> Self {
        Self::default().sub(*self).unwrap()
    }
}

pub trait Trig {
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
}

macro_rules! trig_func {
    ($name:ident, $target:ty, $as:ty) => {
        fn $name(&self) -> Self {
            <$as>::$name(*self as $as) as $target
        }
    }
}

macro_rules! impl_trig {
    ($target:ty, $as:ty) => {
        impl Trig for $target {
            trig_func!(sin, $target, $as);
            trig_func!(cos, $target, $as);
            trig_func!(tan, $target, $as);
        }
    }
}

impl_trig!(f32, f32);
impl_trig!(f64, f64);