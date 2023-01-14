use std::collections::HashMap;

use crate::{compiler::Token, Number, Value};

pub struct ExprEnv<T: Number, const N: usize> {
    named_tokens: HashMap<String, Token<T, N>>,
}

impl<T: Number, const N: usize> ExprEnv<T, N> {
    pub fn new(var_names: [String; N]) -> ExprEnv<T, N> {
        ExprEnv {
            named_tokens: var_names
                .iter()
                .enumerate()
                .map(|(index, elem)| (elem.clone(), Token::Value(Value::Variable(index))))
                .collect(),
        }
    }

    pub(crate) fn get(&self, name: &str) -> Option<&Token<T, N>> {
        self.named_tokens.get(name)
    }
}
