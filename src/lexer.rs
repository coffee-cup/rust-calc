use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Times,
    Divide,
    Pow,
    Integer(i64),
}

pub struct Lexer<'a> {
    iter: Peekable<std::str::Chars<'a>>,
}

pub fn lex(input: &String) -> Vec<Token> {
    Lexer::new(input).collect()
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        Lexer {
            iter: input.chars().peekable(),
        }
    }

    fn advance(&mut self, t: Token) -> Option<Token> {
        self.iter.next();
        Some(t)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // ignore whitespace
        while let Some(' ') = self.iter.peek() {
            self.iter.next();
        }

        if let Some(c) = self.iter.peek() {
            match c {
                '(' => self.advance(Token::LParen),
                ')' => self.advance(Token::RParen),
                '*' => self.advance(Token::Times),
                '/' => self.advance(Token::Divide),
                '+' => self.advance(Token::Plus),
                '-' => self.advance(Token::Minus),
                '^' => self.advance(Token::Pow),
                '0'...'9' => {
                    let mut number = self
                        .iter
                        .next()
                        .unwrap()
                        .to_string()
                        .parse::<i64>()
                        .expect("Error parsing number.");

                    while let Some(Ok(digit)) =
                        self.iter.peek().map(|c| c.to_string().parse::<i64>())
                    {
                        number = number * 10 + digit;
                        self.iter.next();
                    }

                    Some(Token::Integer(number))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_empty_parens() {
        let tokens = lex(&"()".to_owned());
        assert_eq!(tokens, vec![Token::LParen, Token::RParen]);
    }

    #[test]
    fn lex_simple_tokens() {
        let tokens = lex(&"*/+-".to_owned());
        assert_eq!(
            tokens,
            vec![Token::Times, Token::Divide, Token::Plus, Token::Minus]
        );
    }

    #[test]
    fn lex_integer() {
        let tokens: Vec<Token> = lex(&"100".to_owned());
        assert_eq!(tokens, vec![Token::Integer(100)]);
    }

    #[test]
    fn lex_math_expression() {
        let tokens = lex(&"(100 + 2) - (-4)".to_owned());
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Integer(100),
                Token::Plus,
                Token::Integer(2),
                Token::RParen,
                Token::Minus,
                Token::LParen,
                Token::Minus,
                Token::Integer(4),
                Token::RParen
            ]
        );
    }
}
