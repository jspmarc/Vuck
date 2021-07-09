// error di sini buat handle error dari pembuatan token/penambahan token

use super::{
    token::{token_type::TokenType, Token},
    Interpreter,
};
use std::io;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,

    // position
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

// const RESERVED_CHARACTERS: &[char] = &[
//     'l', 'h', 'j', 'k', '+', '-', '*', '/', '%', 'i', 'I', 'p', 'P', ':', 'q', ',', 'F', '.', 'T',
//     '|', ' ', '\t', '\r', '\n',
// ];

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
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

        // End of file
        if let Err(err) = self.add_token(TokenType::Eof, None) {
            return Err(err);
        }

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
                    self.start = self.current;
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
            'F' => self.add_token(TokenType::LoopEnd, None),
            ':' => {
                if self.peek() == 'q' {
                    self.advance();

                    self.add_token(TokenType::Eof, None)
                } else {
                    Interpreter::error(self.line, self.column, "Karakter/perintah invalid.");
                    Ok(())
                }
            }
            '.' => self.add_token(TokenType::ConditionalStart, None),
            'T' => self.add_token(TokenType::ConditionalEnd, None),
            '|' => self.add_token(TokenType::ConditionalElse, None),
            ' ' | '\t' | '\r' => Ok(()),
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(())
            }
            _ => {
                Interpreter::error(self.line, self.column, "Karakter/perintah invalid");
                Ok(())
            }
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
        self.column += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, tok_type: TokenType, literal: Option<i32>) -> io::Result<()> {
        let lexeme = self.source[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>();

        let tok = match Token::new(tok_type, lexeme, literal, self.line, self.column) {
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

    fn number(&mut self) -> io::Result<()> {
        let first_char = self.peek();
        if first_char != '-' && !is_digit(first_char) {
            Interpreter::error(self.line, self.column + 1, "Harusnya angka atau '-'");
            return Ok(());
        } else {
            self.advance();
        }

        while is_digit(self.peek()) {
            self.advance();
        }

        let num = match self.source[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>()
            .parse::<i32>()
        {
            Ok(num) => num,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Gagal membaca angka")),
        };

        self.add_token(TokenType::Number, Some(num))
    }
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}
