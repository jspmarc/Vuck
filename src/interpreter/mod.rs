extern crate lazy_static;

mod runner;
mod scanner;
mod token;

use super::exit_codes;
use lazy_static::lazy_static;
use runner::run;
use std::{
    fs::File,
    io,
    io::{prelude::*, ErrorKind},
    sync::atomic::{AtomicBool, Ordering},
};

lazy_static! {
    pub static ref HAD_ERROR: AtomicBool = AtomicBool::new(false);
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Interpreter {
    pub fn interpret(&mut self, args: &[String]) -> i8 {
        match args.len() {
            1 => match self.run_repl() {
                Ok(_) => exit_codes::EX_OK,
                Err(e) => {
                    eprintln!("Gagal saat membaca input atau menuliskan output: {:#?}", e);
                    exit_codes::EX_IOERR
                }
            },
            2 => match self.run_file(&args[1]) {
                Ok(_) => exit_codes::EX_OK,
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("File {} tidak ditemukan", &args[1]);
                        exit_codes::EX_NOINPUT
                    }
                    _ => {
                        eprintln!("{}", err);
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
        let mut source = String::new();

        while let Ok(len) = buf_reader.read_line(&mut line) {
            if len == 0 {
                break;
            }

            source.push_str(&line);
            line = String::new();
        }

        run(&source).unwrap();

        if HAD_ERROR.load(Ordering::SeqCst) {
            return Err(io::Error::new(ErrorKind::Other, "Gagal interpretasi file."));
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

            run(line.as_str()).unwrap();

            HAD_ERROR.fetch_and(false, Ordering::SeqCst);
            line = "".to_string();
        }

        Ok(())
    }
}

impl Interpreter {
    pub fn report(line: usize, col: usize, whr: &str, message: &str) {
        HAD_ERROR.fetch_or(true, Ordering::SeqCst);
        eprintln!("[Line {} Column {}] Error {}: {}", line, col, whr, message);
    }

    pub fn error(line: usize, col: usize, message: &str) {
        Interpreter::report(line, col, &"".to_string(), message)
    }
}
