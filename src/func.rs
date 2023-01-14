use std::{marker::PhantomData, rc::Rc};

use crate::{EvalError, Number, Value};

pub(crate) trait CustomFunc<T: Number, const N: usize, const A: usize>: 'static + Fn([T; A]) -> T {}
impl<T: Number, const N: usize, const A: usize, F: 'static + Fn([T; A]) -> T> CustomFunc<T, N, A> for F {}

#[derive(Clone)]
pub(crate) struct Function<T: Number, const N: usize> {
    func: Rc<dyn Fn(&[T]) -> T>,
    num: PhantomData<T>,
}

impl<T: Number, const N: usize> Function<T, N> {
    pub(crate) fn new<const A: usize, F: CustomFunc<T, N, A>>(f: F) -> Function<T, N> {
        let boxed = Rc::new(move |args: &[T]| {
            let args: [T; A] = args.try_into().expect("Incorrect argument count");
            f(args)
        });
        Function {
            func: boxed,
            num: PhantomData::default(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct FunctionInvoke<T: Number, const N: usize> {
    func: Function<T, N>,
    args: Vec<Value<T, N>>,
}

impl<T: Number, const N: usize> FunctionInvoke<T, N> {
    pub(crate) fn invoke(&self, vars: &[T; N]) -> Result<T, EvalError> {
        let evaluated = self.args
            .iter().map(|v| v.evaluate(vars))
            .collect::<Result<Vec<T>, EvalError>>()?;
        Ok((self.func.func)(&evaluated))
    }
}
