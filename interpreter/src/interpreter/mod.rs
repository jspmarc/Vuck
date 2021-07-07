mod error;
mod scanner;
mod tokens;

use crate::exit_codes;
use std::{
    fs::File,
    io,
    io::{prelude::*, ErrorKind},
    result::Result,
};

#[derive(Debug)]
pub enum ErrorType {
    IOError(io::Error),
    InterpreterError(String),
}

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
                Err(err) => match err {
                    ErrorType::IOError(e) => match e.kind() {
                        ErrorKind::NotFound => {
                            eprintln!("File {} tidak ditemukan", args[1]);
                            exit_codes::EX_NOINPUT
                        }
                        others => {
                            eprintln!("Terjadi kesalahan saat membaca file:\n{:#?}.", others);
                            exit_codes::EX_IOERR
                        }
                    },
                    ErrorType::InterpreterError(msg) => {
                        eprintln!("{}", msg);
                        exit_codes::EX_SOFTWARE
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
    fn run_file(&self, file_path: &str) -> Result<(), ErrorType> {
        let f = match File::open(file_path) {
            Ok(f) => f,
            Err(err) => return Err(ErrorType::IOError((err))),
        };
        let mut buf_reader = io::BufReader::new(f);
        let mut line = String::new();

        while let Ok(len) = buf_reader.read_line(&mut line) {
            if len == 0 {
                break;
            }

            run(&line);

            line = String::new(); // kalo ga diginiin ntar dia malah append string
        }

        if self.had_error {
            return Err(ErrorType::InterpreterError("Some error".to_string()));
        }

        Ok(())
    }

    #[allow(unreachable_code)]
    fn run_repl(&mut self) -> Result<(), ErrorType> {
        let mut line = String::new();
        let stdin = io::stdin();

        loop {
            print!("> ");
            if let Err(err) = io::stdout().flush() {
                return Err(ErrorType::IOError(err));
            }
            if let Err(err) = stdin.read_line(&mut line) {
                return Err(ErrorType::IOError(err));
            }

            // sementara :q\n dulu
            if line.eq(":q\n") {
                return Ok(());
            }

            run(line.as_str());

            self.had_error = false;
            line = "".to_string();
        }

        Ok(())
    }
}

fn run(source: &str) {
    println!("{:?}", source);

    // private static void run(String source) {
    // Scanner scanner = new Scanner(source);
    // List<Token> tokens = scanner.scanTokens();

    // // For now, just print the tokens.
    // for (Token token : tokens) {
    //     System.out.println(token);
    // }
    // }
}
