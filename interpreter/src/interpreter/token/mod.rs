pub mod position;
pub mod token_type;

use std::{
    any::{Any, TypeId},
    io,
};
use token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    tok_type: TokenType,
    lexeme: String,
    literal: Box<dyn Any>, // cursed awkeowake
    // position: u64, // 0xllllLLLLccccCCCC (l or L is line, c or C is column)
    // ribet kalo nyimpen kolom jg, mager
    line: usize,
}

impl Token {
    pub fn new(
        tok_type: TokenType,
        lexeme: String,
        literal: Box<dyn Any>,
        line: usize,
    ) -> io::Result<Self> {
        let literal_id = (&*literal).type_id();
        if literal_id != TypeId::of::<i32>() && literal_id != TypeId::of::<&str>() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "literal harus string atau signed 4 bytes integer",
            ));
        }
        Ok(Self {
            tok_type,
            lexeme,
            literal,
            line,
        })
    }
}
