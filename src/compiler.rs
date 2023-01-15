use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::env::ExprEnv;
use crate::func::Function;
use crate::func::FunctionInvoke;
use crate::{parser::*, EvalError, Number, Value};

fn get_operator<T: Number, const N: usize>(c: char) -> Option<Token<T, N>> {
    use Token::*;
    match c {
        '+' => Some(BinaryOperator(0, |a, b| {
            a.add(b).ok_or(EvalError::Overflow)
        })),
        '-' => Some(BinaryOperator(0, |a, b| {
            a.sub(b).ok_or(EvalError::Overflow)
        })),
        '*' => Some(BinaryOperator(1, |a, b| {
            a.mul(b).ok_or(EvalError::Overflow)
        })),
        '/' => Some(BinaryOperator(1, |a, b| {
            a.div(b).ok_or(EvalError::DivideByZero)
        })),
        '%' => Some(BinaryOperator(1, |a, b| {
            a.rem(b).ok_or(EvalError::DivideByZero)
        })),
        '^' => Some(BinaryOperator(2, |a, b| a.pow(b))),
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Token<T: Number, const N: usize> {
    Value(Value<T, N>),
    BinaryOperator(usize, fn(T, T) -> Result<T, EvalError>),
    Function(Function<T, N>),
}

impl<T: Number, const N: usize> Token<T, N> {
    fn is_operator(&self) -> bool {
        matches!(self, Self::BinaryOperator(_, _))
    }

    fn get_priority(&self) -> usize {
        match self {
            Self::BinaryOperator(priority, _) => *priority,
            _ => 0,
        }
    }
}

pub(crate) struct ExpressionCompiler<'a, T: Number, const N: usize> {
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
    pub fn compile(s: impl Into<String>, env: ExprEnv<T, N>) -> Result<Value<T, N>, ParserError> {
        let string: String = s.into();
        let chars: Vec<char> = string.chars().filter(|c| !c.is_whitespace()).collect();
        let mut compiler = ExpressionCompiler {
            parser: ParserState {
                source: &chars,
                pos: 0,
            },
            env,
            num_type: Default::default(),
        };
        compiler.parse_expression(None)
    }

    fn parse_expression(&mut self, terminator: Option<char>) -> Result<Value<T, N>, ParserError> {
        let mut tokens = Vec::new();
        tokens.push(self.parse_term_neg()?);
        while self.peek().is_some() && self.peek() != terminator {
            let op = self.parse_binary_operator()?;
            tokens.push(op);
            tokens.push(self.parse_term_neg()?);
        }
        if self.peek() != terminator {
            return Err(ParserError::ExpectedChar(
                self.pos,
                terminator.unwrap_or_default(),
            ));
        }
        self.pos += 1;
        let stack = Self::shunting_yard(tokens)?;
        Self::reduce_stack(stack)
    }

    fn shunting_yard(tokens: Vec<Token<T, N>>) -> Result<VecDeque<Token<T, N>>, ParserError> {
        let mut ops = VecDeque::new();
        let mut stack = VecDeque::new();
        for token in tokens {
            if token.is_operator() {
                while ops.back().map_or(false, |op: &Token<T, N>| {
                    token.get_priority() <= op.get_priority()
                }) {
                    stack.push_back(ops.pop_back().unwrap());
                }
                ops.push_back(token);
            } else {
                stack.push_back(token);
            }
        }
        stack.extend(ops.into_iter().rev());
        Ok(stack)
    }

    fn reduce_stack(mut stack: VecDeque<Token<T, N>>) -> Result<Value<T, N>, ParserError> {
        let value = Self::compile_value(&mut stack)?;
        if stack.is_empty() {
            Ok(value)
        } else {
            Err(ParserError::DanglingValue)
        }
    }

    fn compile_value(stack: &mut VecDeque<Token<T, N>>) -> Result<Value<T, N>, ParserError> {
        match stack.pop_back() {
            Some(Token::Value(val)) => Ok(val),
            Some(Token::BinaryOperator(_, op)) => {
                let right = Self::compile_value(stack)?;
                let left = Self::compile_value(stack)?;
                Ok(Value::BinaryOperation(op, Box::new([left, right])))
            }
            _ => Err(ParserError::NoValue),
        }
    }

    fn parse_name(&mut self) -> Result<String, ParserError> {
        self.take_while("name", |c| c.is_alphabetic())
    }

    fn parse_number(&mut self) -> Result<T, ParserError> {
        self.take_while("number", |c| c.is_ascii_digit() || c == '.')?
            .parse()
            .map_err(|_| ParserError::ExpectedToken(self.pos, "number"))
    }

    fn parse_binary_operator(&mut self) -> Result<Token<T, N>, ParserError> {
        self.advance()
            .and_then(get_operator)
            .ok_or(ParserError::ExpectedToken(self.pos, "operator"))
    }

    fn parse_term_neg(&mut self) -> Result<Token<T, N>, ParserError> {
        let negative = self.check_char('-');
        let term = self.parse_term()?;
        if !negative {
            return Ok(term);
        }
        if let Token::Value(val) = term {
            Ok(Token::Value(Value::UnaryOperation(
                |x| Ok(x.neg()),
                Box::new(val),
            )))
        } else {
            Err(ParserError::ExpectedToken(self.pos, "term"))
        }
    }

    fn parse_function(&mut self, function: Function<T, N>) -> Result<Token<T, N>, ParserError> {
        self.assert_char('(')?;
        let args = if function.args == 0 {
            self.assert_char(')')?;
            vec![]
        } else {
            let mut args = (0..function.args - 1)
                .map(|_| self.parse_expression(Some(',')))
                .collect::<Result<Vec<Value<T, N>>, ParserError>>()?;
            args.push(self.parse_expression(Some(')'))?);
            args
        };
        Ok(Token::Value(Value::FunctionInvoke(FunctionInvoke::new(
            function, args,
        ))))
    }

    fn parse_term(&mut self) -> Result<Token<T, N>, ParserError> {
        match self.peek() {
            Some('0'..='9') => Ok(Token::Value(Value::Constant(self.parse_number()?))),
            Some('(') => {
                self.pos += 1;
                self.parse_expression(Some(')')).map(|e| Token::Value(e))
            }
            _ => {
                let name = self.parse_name()?;
                let value = self
                    .env
                    .get(&name)
                    .ok_or(ParserError::ExpectedToken(self.pos, "name"))?;
                if let Token::Function(func) = value {
                    self.parse_function(func.clone())
                } else {
                    Ok(value.clone())
                }
            }
        }
    }
}
