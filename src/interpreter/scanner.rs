use super::token::{token_type::TokenType, Token};
use std::{any::Any, io, iter::FromIterator};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,

    // position
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> io::Result<&[Token]> {
        while !self.is_at_end() {
            self.start = self.current;

            match self.scan_token() {
                Ok(_) => {}
                Err(err) => return Err(err),
            };
        }

        let tok = match Token::new(
            TokenType::Eof,
            ":q".to_string(),
            Some(Box::new(":q")),
            self.line,
        ) {
            Ok(tok) => tok,
            Err(err) => return Err(err),
        };
        self.tokens.push(tok);

        Ok(self.tokens.as_slice())
    }

    fn scan_token(&mut self) -> io::Result<()> {
        let res = match self.advance() {
            'l' => self.add_token(TokenType::PointerRight, None),
            'h' => self.add_token(TokenType::PointerLeft, None),
            'j' => self.add_token(TokenType::StackPop, None),
            'k' => {
                if let Err(e) = self.add_token(TokenType::StackPush, None) {
                    Err(e)
                } else {
                    self.number()
                }
            }
            '+' => self.add_token(TokenType::MathAdd, None),
            '-' => self.add_token(TokenType::MathSubtract, None),
            '*' => self.add_token(TokenType::MathMultiply, None),
            '/' => self.add_token(TokenType::MathDivide, None),
            '%' => self.add_token(TokenType::MathModulo, None),
            'i' => self.add_token(TokenType::ReadNumber, None),
            'I' => self.add_token(TokenType::ReadAscii, None),
            'p' => self.add_token(TokenType::WriteNumber, None),
            'P' => self.add_token(TokenType::WriteAscii, None),
            ',' => self.add_token(TokenType::LoopStart, None),
            'F' => {
                if let Err(e) = self.add_token(TokenType::LoopEnd, None) {
                    Err(e)
                } else {
                    let res = self.advance();
                    self.add_token(TokenType::LoopMark, Some(Box::new(res)))
                }
            }
            ' ' | '\t' | '\r' | '\n' => Ok(()),
            _ => Ok(()),
        };

        if let Err(e) = res {
            Err(e)
        } else {
            Ok(())
        }
    }
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, tok_t: TokenType, literal: Option<Box<dyn Any>>) -> io::Result<()> {
        let text = self.source[self.start..self.current].to_vec();
        let tok = match Token::new(tok_t, String::from_iter(text), literal, self.line) {
            Ok(tok) => tok,
            Err(err) => return Err(err),
        };

        self.tokens.push(tok);

        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn is_digit(c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn number(&mut self) -> io::Result<()> {
        let mut calon_angka = String::new();
        let mut c = self.advance();

        while c != '.' {
            calon_angka.push(c);
            c = self.advance();
        }

        // c == '.'
        self.current += 1;

        // tambahin dlu token angkanya

        Ok(())
    }
}