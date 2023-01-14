use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::env::ExprEnv;
use crate::linked_list::LinkedList;
use crate::linked_list::Node;
use crate::{parser::*, EvalError, Number, Value};

fn get_operator<T: Number, const N: usize>(c: char) -> Option<Token<T, N>> {
    use Token::*;
    match c {
        '+' => Some(BinaryOperator(0, |a, b| {
            a.checked_add(&b).ok_or(EvalError::Overflow)
        })),
        '-' => Some(BinaryOperator(0, |a, b| {
            a.checked_sub(&b).ok_or(EvalError::Overflow)
        })),
        '*' => Some(BinaryOperator(1, |a, b| {
            a.checked_mul(&b).ok_or(EvalError::Overflow)
        })),
        '/' => Some(BinaryOperator(1, |a, b| {
            a.checked_div(&b).ok_or(EvalError::DivideByZero)
        })),
        '%' => Some(BinaryOperator(1, |a, b| {
            a.checked_rem(&b).ok_or(EvalError::DivideByZero)
        })),
        '^' => Some(BinaryOperator(2, |a, b| a.pow(b))),
        _ => None,
    }
}

#[derive(Clone)]
pub(crate) enum Token<T: Number, const N: usize> {
    Value(Value<T, N>),
    BinaryOperator(usize, fn(T, T) -> Result<T, EvalError>),
    UnaryOperator(fn(T) -> Result<T, EvalError>),
}

struct ExpressionCompiler<'a, T: Number, const N: usize> {
    parser: ParserState<'a>,
    env: ExprEnv<T, N>,
    num_type: PhantomData<T>,
}

impl<'a, T: Number, const N: usize> Deref for ExpressionCompiler<'a, T, N> {
    type Target = ParserState<'a>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}

impl<'a, T: Number, const N: usize> DerefMut for ExpressionCompiler<'a, T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

impl<'a, T: Number, const N: usize> ExpressionCompiler<'a, T, N> {
    fn sort_operators(list: &LinkedList<Token<T, N>>) -> [Vec<Node<Token<T, N>>>; 3] {
        let mut operators: [Vec<_>; 3] = Default::default();
        let mut node = list.head();
        while let Some(inner) = node {
            if let Node {
                data: Token::BinaryOperator(priority, _),
                ..
            } = inner
            {
                operators[*priority].push(inner.own());
            }
            node = inner.next();
        }
        operators
    }

    fn reduce_tokens(
        mut list: LinkedList<Token<T, N>>,
        operators: [Vec<Node<Token<T, N>>>; 3],
    ) -> Result<LinkedList<Token<T, N>>, ParserError> {
        operators.iter().rev().flatten().try_for_each(|op| {
            let (left, right) = list
                .remove_prev(op)
                .zip(list.remove_next(op))
                .ok_or(ParserError::MissingOperand)?;
            if let (
                &Token::BinaryOperator(_, func),
                Token::Value(left_value),
                Token::Value(right_value),
            ) = (&op.data, left, right)
            {
                op.own().data = Token::Value(Value::BinaryOperation(
                    func,
                    Box::new([left_value, right_value]),
                ));
                Ok(())
            } else {
                Err(ParserError::MissingOperand)
            }
        })?;
        Ok(list)
    }

    fn compile_value(list: LinkedList<Token<T, N>>) -> Result<Value<T, N>, ParserError> {
        let operators = Self::sort_operators(&list);
        let list = Self::reduce_tokens(list, operators)?;
        if list.len() != 1 {
            return Err(ParserError::DanglingValue);
        }
        let compiled = list.head().unwrap().own().data;
        if let Token::Value(val) = compiled {
            Ok(val)
        } else {
            Err(ParserError::NoValue)
        }
    }

    fn parse_name(&mut self) -> Result<String, ParserError> {
        self.take_while("name", |c| c.is_alphabetic())
    }

    fn parse_number(&mut self) -> Result<T, ParserError> {
        self.take_while("number", |c| c.is_ascii_digit() || c == '.')?
            .parse()
            .map_err(|_| ParserError::ExpectedToken("number"))
    }

    fn parse_binary_operator(&mut self) -> Result<Token<T, N>, ParserError> {
        self.advance()
            .and_then(get_operator)
            .ok_or(ParserError::ExpectedToken("operator"))
    }

    fn parse_term_neg(&mut self) -> Result<Token<T, N>, ParserError> {
        let negative = self.check_char('-');
        let term = self.parse_term()?;
        if !negative {
            return Ok(term);
        }
        if let Token::Value(val) = term {
            Ok(Token::Value(Value::UnaryOperation(
                |x| Ok(T::default() - x),
                Box::new(val),
            )))
        } else {
            Err(ParserError::ExpectedToken("term"))
        }
    }

    fn parse_term(&mut self) -> Result<Token<T, N>, ParserError> {
        if matches!(self.peek(), Some('0'..='9')) {
            return Ok(Token::Value(Value::Constant(self.parse_number()?)));
        }
        let name = self.parse_name()?;
        let value = self
            .env
            .get(&name)
            .ok_or(ParserError::ExpectedToken("name"))?;
        Ok(value.clone())
    }
}
