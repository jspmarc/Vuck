mod executor;
mod helper;

use super::{
    scanner::Scanner,
    token::{token_type::TokenType, Token},
    HAD_ERROR,
};
use std::{io, sync::atomic::Ordering};

// buat nyimpen state
struct Runner {
    stack: Vec<i32>,
    ptr_idx: i32,
    had_error: bool,
    count_conditional: isize,
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
        let _ = self.the_actual_runner_lol(toks);

        Ok(())
    }

    fn the_actual_runner_lol(&mut self, toks: &[Token]) -> io::Result<()> {
        let mut tok_iter = toks.iter();
        let mut i: usize = 1;

        while let Some(tok) = tok_iter.next() {
            match tok.get_tok_type() {
                TokenType::Eof => return Ok(()),
                TokenType::PointerLeft => self.pointer_left(tok),
                TokenType::PointerRight => self.pointer_right(tok),
                TokenType::StackPush => {
                    let num_tok = tok_iter.next().unwrap();
                    i += 1;
                    self.stack_push(num_tok)
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
                TokenType::LoopStart => {
                    if self.is_stack_empty() {
                        self.error(tok, "Tidak bisa memulai loop jika stack kosong");
                        return Ok(());
                    }

                    // cari akhir di mana
                    let start_idx = i + 1;
                    let end_idx = self.get_loop_end_idx(toks, start_idx);

                    // nge-run kode loop
                    // eksekusi sampe stack-nya 0
                    while !self.is_at_loop_end() {
                        let _ = self.the_actual_runner_lol(&toks[(start_idx - 1)..end_idx]);
                    }

                    // skip to loop end
                    let mut tmp_tok = tok_iter.next().unwrap();
                    while *tmp_tok.get_tok_type() != TokenType::LoopEnd || i != (end_idx - 1) {
                        tmp_tok = tok_iter.next().unwrap();
                        i += 1;
                    }
                    i += 1;
                }
                TokenType::LoopEnd => {
                    if self.is_at_loop_end() {
                        return Ok(());
                    }
                }
                TokenType::ConditionalStart => println!("Masuk ke conditional"),
                TokenType::ConditionalElse => println!("Branch else"),
                TokenType::ConditionalEnd => println!("Akhir conditional"),
                TokenType::Number => self.error(tok, "Kok tiba-tiba angka, mas/mba!"),
            }

            i += 1;
        }

        Ok(())
    }
}

pub fn run(source: &str) -> io::Result<()> {
    let mut runner = Runner {
        stack: vec![],
        ptr_idx: -1,
        had_error: HAD_ERROR.load(Ordering::SeqCst),
        count_conditional: 0,
    };

    runner.run(source)
}
