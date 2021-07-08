#[derive(Debug)]
pub enum TokenType {
    // pointer movement
    PointerLeft,
    PointerRight,

    // stack push and pop
    StackPush,
    StackPop,

    // input and output
    ReadNumber,
    ReadAscii,
    WriteNumber,
    WriteAscii,

    // Mathematical operation
    MathAdd,
    MathMultiply,
    MathSubtract,
    MathDivide,
    MathModulo,

    // loop
    LoopStart,
    LoopEnd,

    // literal
    Number,
    LoopMark,

    Eof, // :q
}
