use std::{fmt::Debug, marker::PhantomData, rc::Rc};

use crate::{EvalError, Number, Value};

pub trait CustomFunc<T: Number, const N: usize, const A: usize>: 'static + Fn([T; A]) -> T {}
impl<T: Number, const N: usize, const A: usize, F: 'static + Fn([T; A]) -> T> CustomFunc<T, N, A>
    for F
{
}

#[derive(Clone)]
pub(crate) struct Function<T: Number, const N: usize> {
    func: Rc<dyn Fn(&[T]) -> T>,
    pub args: usize,
    num: PhantomData<T>,
}

impl<T: Number, const N: usize> Debug for Function<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Function")
    }
}

impl<T: Number, const N: usize> Function<T, N> {
    pub(crate) fn new<const A: usize, F: CustomFunc<T, N, A>>(f: F) -> Function<T, N> {
        let boxed = Rc::new(move |args: &[T]| {
            let args: [T; A] = args.try_into().expect("Incorrect argument count");
            f(args)
        });
        Function {
            func: boxed,
            args: A,
            num: PhantomData::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FunctionInvoke<T: Number, const N: usize> {
    pub func: Function<T, N>,
    pub args: Vec<Value<T, N>>,
}

impl<T: Number, const N: usize> FunctionInvoke<T, N> {
    pub fn new(func: Function<T, N>, args: Vec<Value<T, N>>) -> FunctionInvoke<T, N> {
        FunctionInvoke { func, args }
    }

    pub fn invoke(&self, vars: &[T; N]) -> Result<T, EvalError> {
        let evaluated = self
            .args
            .iter()
            .map(|v| v.evaluate(vars))
            .collect::<Result<Vec<T>, EvalError>>()?;
        Ok((self.func.func)(&evaluated))
    }
}
