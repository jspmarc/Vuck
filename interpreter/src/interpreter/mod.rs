mod error;
mod scanner;
mod token;

use super::exit_codes;
use scanner::Scanner;
use std::{
    fs::File,
    io,
    io::{prelude::*, ErrorKind},
};

pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { had_error: false }
    }
}

impl Interpreter {
    pub fn interpret(&mut self, args: &[String]) -> i32 {
        match args.len() {
            1 => match self.run_repl() {
                Ok(_) => exit_codes::EX_OK,
                Err(e) => {
                    eprintln!("Gagal saat membaca input atau menuliskan output:\n{:#?}", e);
                    exit_codes::EX_IOERR
                }
            },
            2 => match self.run_file(&args[1]) {
                Ok(_) => exit_codes::EX_OK,
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("File {} tidak ditemukan", args[1]);
                        exit_codes::EX_NOINPUT
                    }
                    others => {
                        eprintln!("Terjadi kesalahan saat membaca file:\n{:#?}.", others);
                        exit_codes::EX_IOERR
                    }
                },
            },
            _ => {
                eprintln!("Penggunaan: {} <path/ke/file>", &args[0]);
                exit_codes::EX_USAGE
            }
        }
    }
}

impl Interpreter {
    fn run_file(&mut self, file_path: &str) -> io::Result<()> {
        let f = match File::open(file_path) {
            Ok(f) => f,
            Err(err) => return Err(err),
        };
        let mut buf_reader = io::BufReader::new(f);
        let mut line = String::new();

        while let Ok(len) = buf_reader.read_line(&mut line) {
            if len == 0 {
                break;
            }

            if let Err(err) = run(line.as_str()) {
                self.had_error = true;
                return Err(err);
            }

            line = String::new();
        }

        if self.had_error {
            return Err(io::Error::new(ErrorKind::Other, "Some error"));
        }

        Ok(())
    }

    #[allow(unreachable_code)]
    fn run_repl(&mut self) -> io::Result<()> {
        let mut line = String::new();
        let stdin = io::stdin();

        loop {
            print!("> ");
            if let Err(err) = io::stdout().flush() {
                return Err(err);
            }
            if let Err(err) = stdin.read_line(&mut line) {
                return Err(err);
            }

            // sementara :q\n dulu
            if line.eq(":q\n") {
                return Ok(());
            }

            if let Err(err) = run(line.as_str()) {
                return Err(err);
            }

            self.had_error = false;
            line = "".to_string();
        }

        Ok(())
    }
}

fn run(source: &str) -> io::Result<()> {
    let mut scanner = Scanner::new(source);
    println!("{:?}", source);

    let toks = match scanner.scan_tokens() {
        Ok(toks) => toks,
        Err(err) => return Err(err),
    };

    for tok in toks {
        println!("{:#?}", tok);
    }

    Ok(())
}
