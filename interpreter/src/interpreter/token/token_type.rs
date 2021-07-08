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

    // Mathematical operation
    ADD,
    MULTIPLY,
    SUBTRACT,
    DIVIDE,
    MODULO,

    // conditional
    CONDITIONAL_START,
    CONDITIONAL_END,

    // literal
    NUMBER,
    CONDITIONAL_MARK,

    EOF, // :q
}
