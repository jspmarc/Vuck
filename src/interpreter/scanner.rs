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

    // stop pas udah nge-scan eof
    has_seen_eof: bool,

    // count loop and conditional. Should be zero at the end of parsing
    // in_loop: bool,
    // in_conditional: bool,
    count_loop: isize,
    count_conditional: isize,
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
            has_seen_eof: false,
            // in_loop: false,
            // in_conditional: false,
            count_conditional: 0,
            count_loop: 0,
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

        if self.is_at_end() {
            if !self.has_seen_eof {
                Interpreter::error(self.line, self.column, "Membutuhkan :q")
            }
            if self.count_conditional != 0 {
                Interpreter::error(
                    self.line,
                    self.column,
                    "Ada conditional belum ditutup atau belum dibuka",
                )
            }
            if self.count_loop != 0 {
                Interpreter::error(
                    self.line,
                    self.column,
                    "Ada loop belum ditutup atau belum dibuka",
                )
            }
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
            ',' => {
                // if self.in_loop {
                //     Interpreter::error(
                //         self.line,
                //         self.column,
                //         "Tidak boleh ada loop di dalam loop",
                //     );
                //     return Ok(());
                // }
                // self.in_loop = true;
                self.count_loop += 1;
                self.add_token(TokenType::LoopStart, None)
            }
            'F' => {
                // if !self.in_loop {
                //     Interpreter::error(self.line, self.column, "Sedang tidak ada di dalam loop");
                //     return Ok(());
                // }
                // self.in_loop = false;
                self.count_loop -= 1;
                self.add_token(TokenType::LoopEnd, None)
            }
            ':' => {
                if self.peek() == 'q' {
                    self.advance();
                    self.has_seen_eof = true;
                    self.add_token(TokenType::Eof, None)
                } else {
                    Interpreter::error(self.line, self.column, "Karakter/perintah invalid");
                    Ok(())
                }
            }
            '.' => {
                // if self.in_conditional {
                //     Interpreter::error(
                //         self.line,
                //         self.column,
                //         "Tidak boleh ada conditonal di dalam conditional",
                //     );
                //     return Ok(());
                // }
                // self.in_conditional = true;
                self.count_conditional += 1;
                self.add_token(TokenType::ConditionalStart, None)
            }
            'T' => {
                // if !self.in_conditional {
                //     Interpreter::error(self.line, self.column, "Tidak sedang di dalam conditonal");
                //     return Ok(());
                // }
                // self.in_conditional = false;
                self.count_conditional -= 1;
                self.add_token(TokenType::ConditionalEnd, None)
            }
            '|' => {
                if self.count_conditional == 0 {
                    Interpreter::error(self.line, self.column, "Tidak sedang di dalam conditional");
                    return Ok(());
                }
                self.add_token(TokenType::ConditionalElse, None)
            }
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
        self.current >= self.source.len() || self.has_seen_eof
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
