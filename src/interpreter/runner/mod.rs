mod executor;
mod helper;

use super::{
    scanner::Scanner,
    token::{token_type::TokenType, Token},
    HAD_ERROR,
};
use std::{io, sync::atomic::Ordering};

struct Runner {
    stack: Vec<i32>,
    ptr_idx: i32,
    had_error: bool,
}

impl Runner {
    /// "main" interpreter runner
    fn run(&mut self, source: &str) -> io::Result<()> {
        let mut scanner = Scanner::new(source);

        // parse
        let toks = match scanner.scan_tokens() {
            Ok(toks) => toks,
            Err(err) => return Err(err),
        };

        if self.had_error {
            return Ok(()); // kaborr
        }

        // run
        let mut tok_iter = toks.iter();
        while let Some(tok) = tok_iter.next() {
            match tok.get_tok_type() {
                TokenType::Eof => {}
                TokenType::PointerLeft => self.pointer_left(tok),
                TokenType::PointerRight => self.pointer_right(tok),
                TokenType::StackPush => {
                    let num_tok = tok_iter.next().unwrap();
                    self.stack_push(num_tok);
                }
                TokenType::StackPop => self.stack_pop(tok),
                TokenType::ReadNumber => self.read_number(),
                TokenType::ReadAscii => self.read_ascii(tok),
                TokenType::WriteNumber => self.write_number(tok),
                TokenType::WriteAscii => self.write_ascii(tok),
                TokenType::MathAdd => self.math(tok, '+'),
                TokenType::MathMultiply => self.math(tok, '*'),
                TokenType::MathSubtract => self.math(tok, '-'),
                TokenType::MathDivide => self.math(tok, '/'),
                TokenType::MathModulo => self.math(tok, '%'),
                TokenType::LoopStart => todo!(),
                TokenType::LoopEnd => todo!(),
                TokenType::ConditionalStart => todo!(),
                TokenType::ConditionalEnd => todo!(),
                TokenType::ConditionalElse => todo!(),
                TokenType::Number => self.error(tok, "Kok tiba-tiba angka anjay?"),
            }
        }

        Ok(())
    }
}

pub fn run(source: &str) -> io::Result<()> {
    let mut runner = Runner {
        stack: vec![],
        ptr_idx: -1,
        had_error: HAD_ERROR.load(Ordering::SeqCst),
    };

    runner.run(source)
}
