mod position;
mod token_type;

use token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    lexeme: String,
    position: u64, // 0xllllLLLLccccCCCC (l or L is line, c or C is column)
    tok_type: TokenType,
}

impl Token {
    pub fn new(lexeme: String, position: u64, tok_type: TokenType) -> Self {
        Self {
            lexeme,
            position,
            tok_type,
        }
    }
}
