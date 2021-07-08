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
