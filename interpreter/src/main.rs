mod exit_codes;
mod interpreter;

use interpreter::Interpreter;
use std::{env, io, process};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let mut interpreter = Interpreter::new();
    let exit_code = interpreter.interpret(&args);

    process::exit(exit_code);
}
