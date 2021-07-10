pub mod token_type;

use std::io;
use token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    tok_type: TokenType,
    lexeme: String,
    literal: Option<i32>,
    position: (usize, usize),
}

impl Token {
    pub fn new(
        tok_type: TokenType,
        lexeme: String,
        literal_opt: Option<i32>,
        line: usize,
        column: usize,
    ) -> io::Result<Self> {
        if let Some(literal) = literal_opt {
            Ok(Self {
                tok_type,
                lexeme,
                literal: Some(literal),
                position: (line, column),
            })
        } else {
            Ok(Self {
                tok_type,
                lexeme,
                literal: None,
                position: (line, column),
            })
        }
    }
}

// getters
impl Token {
    pub fn get_tok_type(&self) -> &TokenType {
        &self.tok_type
    }
    pub fn get_lexeme(&self) -> &String {
        &self.lexeme
    }
    pub fn get_literal(&self) -> &Option<i32> {
        &self.literal
    }
    pub fn get_line(&self) -> &usize {
        &self.position.0
    }
    pub fn get_column(&self) -> &usize {
        &self.position.1
    }
}
