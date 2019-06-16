use std::iter::Peekable;
use std::slice::Iter;

use crate::lexer::*;

// nud: null denotation
// led: left denotation

#[derive(Debug, PartialEq)]
pub enum Expr {
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Literal(i64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Pow,
    Mul,
    Div,
    Add,
    Sub,
}

fn token_to_binop(token: Token) -> Result<BinaryOp, String> {
    match token {
        Token::Pow => Ok(BinaryOp::Pow),
        Token::Times => Ok(BinaryOp::Mul),
        Token::Divide => Ok(BinaryOp::Div),
        Token::Plus => Ok(BinaryOp::Add),
        Token::Minus => Ok(BinaryOp::Sub),
        _ => Err("no binop for token".to_owned()),
    }
}

pub fn parse_and_lex(input: &String) -> Result<Expr, String> {
    let tokens = lex(&input);
    parse(&tokens)
}

pub fn parse(tokens: &Vec<Token>) -> Result<Expr, String> {
    let mut parser = Parser::new(tokens.iter());
    let ast = parser.parse_expr();

    if let None = parser.input.peek() {
        ast
    } else {
        Err("input has not fully been consumed".to_owned())
    }
}

type Tokens<'a> = Peekable<Iter<'a, Token>>;

struct Parser<'a> {
    input: Tokens<'a>,
}

impl Token {
    // null denotation
    // does not care about the token on the left
    fn nud(&self, parser: &mut Parser) -> Result<Expr, String> {
        match *self {
            Token::Integer(n) => Ok(Expr::Literal(n)),
            Token::Minus => {
                let e = parser.expression(100)?;
                Ok(Expr::Unary(UnaryOp::Neg, Box::new(e)))
            }
            Token::LParen => {
                let e = parser.expression(0)?;
                if let Some(Token::RParen) = parser.input.next() {
                    Ok(e)
                } else {
                    Err("unbalanced parens".to_owned())
                }
            }
            _ => Err("expecting literal".to_owned()),
        }
    }

    // left denotation
    // does care about the token on the left
    fn led(&self, parser: &mut Parser, lhs: Expr) -> Result<Expr, String> {
        match *self {
            Token::Times | Token::Divide | Token::Plus | Token::Minus => {
                let rhs = parser.expression(self.lbp())?;
                let op = token_to_binop(self.clone()).unwrap();
                Ok(Expr::Binary(Box::new(lhs), op, Box::new(rhs)))
            }
            Token::Pow => {
                let rhs = parser.expression(self.lbp() - 1)?;
                Ok(Expr::Binary(Box::new(lhs), BinaryOp::Pow, Box::new(rhs)))
            }
            _ => Err("expecting operator".to_owned()),
        }
    }

    // left binding power
    // precedence of each operator
    fn lbp(&self) -> u32 {
        match self {
            Token::Pow => 30,
            Token::Times | Token::Divide => 20,
            Token::Plus | Token::Minus => 10,
            _ => 0,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: Iter<'a, Token>) -> Parser<'a> {
        Parser {
            input: input.peekable(),
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.expression(0)
    }

    fn expression(&mut self, rbp: u32) -> Result<Expr, String> {
        let mut left = self.parse_nud()?;
        while self.next_binds_tighter_than(rbp) {
            left = self.parse_led(left)?;
        }

        Ok(left)
    }

    fn next_binds_tighter_than(&mut self, rbp: u32) -> bool {
        self.input.peek().map_or(false, |t| t.lbp() > rbp)
    }

    fn parse_nud(&mut self) -> Result<Expr, String> {
        self.input
            .next()
            .map_or(Err("incomplete".to_owned()), |t| t.nud(self))
    }

    fn parse_led(&mut self, expr: Expr) -> Result<Expr, String> {
        self.input
            .next()
            .map_or(Err("incomplete".to_owned()), |t| t.led(self, expr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal_expr() {
        let ast = parse_and_lex(&"2".to_owned());
        assert_eq!(ast, Ok(Expr::Literal(2)))
    }

    #[test]
    fn parse_addition_expr() {
        let ast = parse_and_lex(&"2 + 1".to_owned());
        assert_eq!(
            ast,
            Ok(Expr::Binary(
                Box::new(Expr::Literal(2)),
                BinaryOp::Add,
                Box::new(Expr::Literal(1))
            ))
        )
    }

    #[test]
    fn parse_paren() {
        let ast = parse_and_lex(&"(1)".to_owned());
        assert_eq!(ast, Ok(Expr::Literal(1)))
    }

    #[test]
    fn basic_precedence() {
        let ast = parse_and_lex(&"1 + 2 * 3".to_owned());
        assert_eq!(
            ast,
            Ok(Expr::Binary(
                Box::new(Expr::Literal(1)),
                BinaryOp::Add,
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(2)),
                    BinaryOp::Mul,
                    Box::new(Expr::Literal(3))
                ))
            ))
        )
    }
}
