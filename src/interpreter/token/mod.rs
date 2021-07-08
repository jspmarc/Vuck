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
    literal: Option<Box<dyn Any>>, // cursed awkeowake
    // position: u64, // 0xllllLLLLccccCCCC (l or L is line, c or C is column)
    // ribet kalo nyimpen kolom jg, mager
    position: (usize, usize),
}

impl Token {
    pub fn new(
        tok_type: TokenType,
        lexeme: String,
        literal_opt: Option<Box<dyn Any>>,
        line: usize,
        column: usize,
    ) -> io::Result<Self> {
        if let Some(literal) = literal_opt {
            let literal_id = (&*literal).type_id();

            // type checking
            if literal_id != TypeId::of::<i32>() && literal_id != TypeId::of::<char>() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "literal harus sebuah karakter atau signed 4 bytes integer",
                ));
            }

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
