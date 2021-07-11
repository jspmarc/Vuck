use super::{Runner, Token};
use std::{io, usize};

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().unwrap();
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

impl Runner {
    pub fn pointer_left(&mut self, tok: &Token) {
        if self.is_stack_empty() {
            self.error(tok, "Stack kosong atau pointer sudah di bawah stack")
        }
        self.ptr_idx -= 1;
    }
    pub fn pointer_right(&mut self, tok: &Token) {
        if self.is_stack_empty() {
            self.error(tok, "Stack kosong atau ointer sudah di atas stack")
        }
        self.ptr_idx += 1;
    }
    pub fn stack_push(&mut self, tok: &Token) {
        let num = tok.get_literal().unwrap();
        self.stack.push(num);
        self.reset_pointer();
    }
    pub fn stack_pop(&mut self, tok: &Token) {
        if self.is_stack_empty() {
            self.error(tok, "Stack kosong")
        }

        self.stack.pop();
        self.reset_pointer();
    }
    pub fn read_number(&mut self) {
        read!(num as i32);
        self.stack.push(num);
        self.reset_pointer();
    }
    pub fn read_ascii(&mut self, tok: &Token) {
        read_str!(s);
        let target = s.as_bytes()[0];
        if !target.is_ascii() {
            self.error(tok, "Yang dibaca bukan karakter ASCII")
        }

        self.stack.push(target as i32);
        self.reset_pointer();
    }
    pub fn write_number(&mut self, tok: &Token) {
        if let Some(num) = self.stack.get(self.ptr_idx as usize) {
            print!("{}", num);
        } else {
            self.error(tok, "Stack kosong")
        }
    }
    pub fn write_ascii(&mut self, tok: &Token) {
        if let Some(num) = self.stack.get(self.ptr_idx as usize) {
            let num = *num as u8;
            let char = char::from_u32(num as u32).unwrap();
            print!("{}", char);
        } else {
            self.error(tok, "Stack kosong")
        }
    }
    pub fn math(&mut self, tok: &Token, operand: char) {
        let (first, second) = {
            if self.stack.len() < 2 {
                self.error(tok, "Jumlah operand di stack kurang");
                return;
            }
            let right = self.stack.pop().unwrap();
            let left = self.stack.pop().unwrap();
            (left, right)
        };

        let res = match operand {
            '+' => first + second,
            '-' => first - second,
            '*' => first * second,
            '/' => first / second,
            '%' => first % second,
            _ => 0, // harusnya ga pernah ke sini
        };
        self.stack.push(res);
        self.reset_pointer();
    }
}
