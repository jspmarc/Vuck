use super::{Runner, Token, TokenType};
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

    pub fn get_loop_end_idx(&mut self, toks: &[Token], start_idx: usize) -> usize {
        let mut i = start_idx;
        let rest_of_code = &toks[start_idx..];
        let mut current_depth = 0;

        for tok in rest_of_code {
            i += 1;
            if *tok.get_tok_type() == TokenType::LoopEnd {
                if current_depth == 0 {
                    break;
                } else {
                    current_depth -= 1;
                }
            } else if *tok.get_tok_type() == TokenType::LoopStart {
                current_depth += 1;
            }
        }

        i
    }

    pub fn get_else_branch_idx(&mut self, toks: &[Token], start_idx: usize) -> usize {
        let mut i = start_idx;
        let rest_of_code = &toks[start_idx..];
        for tok in rest_of_code {
            i += 1;
            if *tok.get_tok_type() == TokenType::ConditionalEnd {
                break;
            }
        }

        i
    }
}
