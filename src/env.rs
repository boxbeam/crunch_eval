use std::collections::HashMap;

use crate::{
    compiler::Token,
    func::{CustomFunc, Function},
    number::Trig,
    Number, Value,
};

pub struct ExprEnv<T: Number, const N: usize> {
    named_tokens: HashMap<String, Token<T, N>>,
}

impl<T: Number> Default for ExprEnv<T, 0> {
    fn default() -> Self {
        ExprEnv {
            named_tokens: HashMap::new(),
        }
    }
}

impl<T: Number, const N: usize> ExprEnv<T, N> {
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
    pub fn with_trig(self) -> Self {
        self.with_func("sin", |[x]: [T; 1]| x.sin())
            .with_func("cos", |[x]: [T; 1]| x.cos())
            .with_func("tan", |[x]: [T; 1]| x.tan())
    }
}
