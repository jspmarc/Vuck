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
            TokenType::EOF,
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
        match self.advance() {};
        Ok(())
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
}
