#[derive(Debug)]
pub enum TokenType {
    // pointer movement
    POINTER_LEFT,
    POINTER_RIGHT,

    // stack push and pop
    STACK_PUSH,
    STACK_POP,

    // input and output
    READ_NUMBER,
    READ_ASCII,
    WRITE_NUMBER,
    WRITE_ASCII,

    // literal
    NUMBER,

    // Mathematical operation
    ADD,
    MULTIPLY,
    SUBTRACT,
    DIVIDE,
    MODULO,

    // conditional
    CONDITIONAL_START,
    CONDITIONAL_END,
    CONDITIONAL_MARK,

    EOF, // :q
}

pub struct Position {
    line: u32,
    column: u32,
}

#[derive(Debug)]
pub struct Token {
    lexeme: String,
    position: u64, // 0xllllLLLLccccCCCC (l or L is line, c or C is column)
    tok_type: TokenType,
}

impl Token {
    pub fn new(lexeme: String, position: u64, tok_type: TokenType) -> Self {
        Self {
            lexeme,
            position,
            tok_type,
        }
    }
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
