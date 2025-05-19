use std::{collections::VecDeque, iter::Peekable, str::Chars};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub enum NapToken {
    Word(String),
    Number(f64),
    Arithmetic(NapArithmeticOp),
    OpenParen,
    CloseParen,
    Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NapArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(thiserror::Error, Debug)]
pub enum TokenTreeParseError {
    #[error("failed to parse expression \"{1}\", cause: {0}")]
    Lexer(LexError, String),
}

#[derive(Debug)]
pub enum NapExpr {
    Call(String, Vec<NapExpr>),
    UnaryOp(NapArithmeticOp, Box<NapExpr>),
    BinOp(NapArithmeticOp, Box<NapExpr>, Box<NapExpr>),
    Identifier(String),
    Number(f64),
}

#[derive(PartialEq, Eq)]
pub enum ExprEndType {
    None,
    CloseParen,
    Comma,
}

impl NapExpr {
    pub fn parse(expression: &str) -> Result<Self, TokenTreeParseError> {
        let mut tokens = VecDeque::new();
        let mut lexer = NapLexer::new(expression);

        while let Some(token) = lexer
            .next()
            .map_err(|err| TokenTreeParseError::Lexer(err, expression.to_string()))?
        {
            tokens.push_back(token);
        }

        Ok(Self::parse_inner(&mut tokens, false).0)
    }

    fn parse_inner(tokens: &mut VecDeque<NapToken>, short_operand: bool) -> (Self, ExprEndType) {
        let mut collected = VecDeque::new();
        let mut expr_stack = Vec::new();

        while let Some(token) = tokens.pop_front() {
            match token {
                NapToken::Arithmetic(op) => {
                    if short_operand && (!collected.is_empty() || !expr_stack.is_empty()) {
                        match expr_stack.pop() {
                            None if collected.is_empty() => (),
                            Some(prev_expr) => {
                                tokens.push_front(NapToken::Arithmetic(op));
                                return (prev_expr, ExprEndType::None);
                            }
                            None => {
                                tokens.push_front(NapToken::Arithmetic(op));
                                return NapExpr::parse_inner(&mut collected, false);
                            }
                        }
                    }

                    let lhs = match expr_stack.pop() {
                        Some(prev_expr) => Some(prev_expr),
                        None if collected.is_empty() => None,
                        None => Some(NapExpr::parse_inner(&mut collected, false).0),
                    };

                    match lhs {
                        Some(lhs) => expr_stack.push(NapExpr::BinOp(
                            op.clone(),
                            Box::new(lhs),
                            Box::new(
                                NapExpr::parse_inner(
                                    tokens,
                                    matches!(op, NapArithmeticOp::Mul | NapArithmeticOp::Div),
                                )
                                .0,
                            ),
                        )),
                        None => {
                            assert!(matches!(op, NapArithmeticOp::Add | NapArithmeticOp::Sub));

                            expr_stack.push(NapExpr::UnaryOp(
                                op,
                                Box::new(NapExpr::parse_inner(tokens, true).0),
                            ))
                        }
                    }
                }
                NapToken::OpenParen => {
                    let (mut inner, mut end) = NapExpr::parse_inner(tokens, false);
                    if let Some(NapToken::Word(name)) = collected.pop_back() {
                        let mut args = Vec::new();

                        loop {
                            args.push(inner);
                            if end != ExprEndType::Comma {
                                if let Some(t) = tokens.pop_front() {
                                    if t != NapToken::Comma {
                                        tokens.push_front(t);
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }

                            (inner, end) = NapExpr::parse_inner(tokens, false);
                        }

                        expr_stack.push(NapExpr::Call(name.clone(), args));
                    } else {
                        expr_stack.push(inner);
                    }
                }
                NapToken::Comma => {
                    return (
                        match expr_stack.pop() {
                            Some(prev_expr) => prev_expr,
                            None => NapExpr::parse_inner(&mut collected, false).0,
                        },
                        ExprEndType::Comma,
                    );
                }
                NapToken::CloseParen => {
                    return (
                        match expr_stack.pop() {
                            Some(prev_expr) => prev_expr,
                            None => NapExpr::parse_inner(&mut collected, false).0,
                        },
                        ExprEndType::CloseParen,
                    );
                }
                _ => collected.push_back(token),
            }
        }

        (
            if let Some(prev_expr) = expr_stack.pop() {
                prev_expr
            } else if collected.len() == 1 {
                match collected.remove(0).unwrap() {
                    NapToken::Word(word) => Self::Identifier(word),
                    NapToken::Number(num) => Self::Number(num),
                    _ => unreachable!(),
                }
            } else {
                panic!("{collected:?}");
            },
            ExprEndType::None,
        )
    }
}

struct NapLexer<'c> {
    chars: Peekable<Chars<'c>>,
}

enum LexerState {
    Word,
    Number,
}

#[derive(thiserror::Error, Debug)]
pub enum LexError {
    #[error("unexpected character at the beginning of token: {0}")]
    UnexpectedChar(char),
}

impl<'c> NapLexer<'c> {
    pub fn new(expression: &'c str) -> Self {
        Self {
            chars: expression.chars().peekable(),
        }
    }

    pub fn next(&mut self) -> Result<Option<NapToken>, LexError> {
        let Some(next_char) = self.chars.peek() else {
            return Ok(None);
        };

        let state = match next_char {
            'A'..='Z' | 'a'..='z' | '_' => LexerState::Word,
            '0'..='9' => LexerState::Number,
            '+' | '-' | '*' | '/' => {
                return Ok(Some(NapToken::Arithmetic(NapArithmeticOp::new(
                    self.chars.next().unwrap(),
                ))));
            }
            '(' => {
                self.chars.next();
                return Ok(Some(NapToken::OpenParen));
            }
            ')' => {
                self.chars.next();
                return Ok(Some(NapToken::CloseParen));
            }
            ',' => {
                self.chars.next();
                return Ok(Some(NapToken::Comma));
            }
            unexpected => return Err(LexError::UnexpectedChar(*unexpected)),
        };

        Ok(Some(match state {
            LexerState::Word => NapToken::Word(
                self.chars
                    .take_while_ref(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))
                    .collect(),
            ),
            LexerState::Number => NapToken::Number(
                self.chars
                    .take_while_ref(|c| matches!(c, '0'..='9' | '.'))
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            ),
        }))
    }
}

impl NapArithmeticOp {
    fn new(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            _ => unreachable!(),
        }
    }
}
