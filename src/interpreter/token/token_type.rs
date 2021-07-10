#[derive(Debug, PartialEq, Eq)]
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

    // conditionals
    ConditionalStart,
    ConditionalEnd,
    ConditionalElse,

    // literal
    Number,

    Eof, // :q
}
