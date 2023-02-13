#[derive(Debug)]
pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    // TODO: Do we want this to return a muable reference since we're going to
    // call methods on it?
    /// Return a brand new, completely untouched, Chunk to be used
    pub fn init() -> Chunk {
        Chunk { code: Vec::new() }
    }

    /// Just a wrapper around Chunk::init() to make the transition to Rust from
    /// C much easier
    pub fn free(&self) -> Chunk {
        Chunk::init()
    }

    // TODO: Is it idiomatic to take in a mutable reference to `self` here?
    /// Store an OpCode instruction in the internal list of codes
    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }
}
