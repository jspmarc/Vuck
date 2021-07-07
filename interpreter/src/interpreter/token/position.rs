pub struct Position {
    line: u32,
    column: u32,
}

impl Position {
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, column: col }
    }
}

impl Position {
    pub fn encode_position(&self) -> u64 {
        let line_64 = self.line as u64;
        let col_64 = self.column as u64;

        (line_64 << (4 * 8)) + col_64
    }
}

impl Position {
    pub fn decode_position(pos: u64) -> Self {
        let line_64 = pos >> (4 * 8);
        let line = line_64 as u32;
        let column = pos as u32;

        Self { line, column }
    }
}
