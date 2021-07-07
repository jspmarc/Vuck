pub fn report(line: i32, col: i32, whr: &str, message: &str) {
    eprintln!("[Line {} Column {}] Error {}: {}", line, col, whr, message);
}

pub fn error(line: i32, col: i32, message: &str) {
    report(line, col, &"".to_string(), message)
}
