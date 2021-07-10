use super::{Runner, Token};
use crate::Interpreter;

impl Runner {
    pub fn is_stack_empty(&self) -> bool {
        self.ptr_idx < 0 || self.stack.is_empty()
    }

    pub fn reset_pointer(&mut self) {
        self.ptr_idx = self.stack.len() as i32 - 1;
    }

    pub fn error(&mut self, tok: &Token, message: &str) {
        Interpreter::error(*tok.get_line(), *tok.get_column(), message);
        self.had_error = true;
    }

    pub fn is_at_loop_end(&mut self) -> bool {
        self.stack[self.ptr_idx as usize] == 0
    }
}
