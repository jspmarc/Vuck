use super::scanner::Scanner;
use std::io;

/// "main" interpreter runner
pub fn run(source: &str) -> io::Result<()> {
    let mut scanner = Scanner::new(source);

    let toks = match scanner.scan_tokens() {
        Ok(toks) => toks,
        Err(err) => return Err(err),
    };

    for tok in toks {
        println!("{:#?}", tok);
    }

    Ok(())
}
