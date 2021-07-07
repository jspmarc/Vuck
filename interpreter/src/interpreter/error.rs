use std::io;

pub fn report(col: i32, whr: &str, message: &str) {
    eprintln!("[Column {}] Error {}: {}", col, whr, message);
}

pub fn error(col: i32, message: &str) {
    report(col, &"".to_string(), message)
}
