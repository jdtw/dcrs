use crate::error::Error::{self, EndOfRegister};
use crate::stack::val::Val;
use std::collections::HashMap;

pub struct Registers {
    reg: HashMap<Val, Vec<Val>>,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            reg: HashMap::new(),
        }
    }

    pub fn push(&mut self, id: Val, val: Val) {
        self.reg.entry(id).or_insert_with(Vec::new).push(val);
    }

    pub fn pop(&mut self, id: &Val) -> Result<Val, Error> {
        self.reg
            .get_mut(id)
            .and_then(|stack| stack.pop())
            .ok_or_else(|| EndOfRegister(id.to_string()))
    }

    pub fn val(&self, id: &Val) -> Result<Val, Error> {
        self.reg
            .get(id)
            .and_then(|stack| stack.last())
            .cloned()
            .ok_or_else(|| EndOfRegister(id.to_string()))
    }

    pub fn iter(&self, id: &Val) -> Option<impl Iterator<Item = &Val>> {
        self.reg.get(id).map(|stack| stack.iter())
    }
}
