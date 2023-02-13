use std::collections::HashMap;

use crate::value::{Value, ValueArray};

#[derive(Debug)]
pub enum OpCode {
    OpConstant(usize),
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    // Use a HashMap here so we don't have to grow the Vec to match the length
    // of `code`.
    pub lines: HashMap<usize, u64>, // HashMap<offset, line number>
    pub constants: ValueArray,
}

impl Chunk {
    // TODO: Do we want this to return a muable reference since we're going to
    // call methods on it?
    /// Return a brand new, completely untouched, Chunk to be used
    pub fn init() -> Chunk {
        Chunk {
            code: Vec::new(),
            lines: HashMap::new(),
            constants: ValueArray::init(),
        }
    }

    /// Just a wrapper around Chunk::init() to make the transition to Rust from
    /// C much easier
    pub fn free(&self) -> Chunk {
        Chunk::init()
    }

    // TODO: Is it idiomatic to take in a mutable reference to `self` here?
    /// Store an OpCode instruction in the internal list of codes
    pub fn write(&mut self, byte: OpCode, line: u64) {
        self.code.push(byte);
        // We need to subtract 1 here since the offset is 0-indexed
        self.lines.insert(self.code.len() - 1, line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);

        // We need to subtract 1 here since the constant offset is 0-index
        self.constants.values.len() - 1
    }
}
