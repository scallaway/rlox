use crate::value::{Value, Values};

// TODO: impl fmt::Display
#[derive(Debug)]
pub(crate) enum OpCode {
    Constant(usize),
    Return,
}

pub(crate) struct Chunk {
    pub(crate) code: Vec<OpCode>,
    pub(crate) constants: Values,
}

impl Chunk {
    pub(crate) fn init() -> Self {
        Self {
            code: Vec::new(),
            constants: Values::init(),
        }
    }

    pub(crate) fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }

    pub(crate) fn add_constant(&mut self, value: Value) -> usize {
        self.constants.values.push(value);

        self.constants.values.len()
    }
}
