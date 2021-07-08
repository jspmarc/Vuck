use super::token::{token_type::TokenType, Token};
use std::io;

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,

    // position
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.as_bytes().to_owned(),
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
            self.current = self.source.len();
        }

        let tok = match Token::new(TokenType::EOF, ":q".to_string(), Box::new(":q"), self.line) {
            Ok(tok) => tok,
            Err(err) => return Err(err),
        };
        self.tokens.push(tok);

        Ok(self.tokens.as_slice())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
