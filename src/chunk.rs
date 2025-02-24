use crate::value::{Value, Values};

// TODO: impl fmt::Display
#[derive(Debug)]
pub(crate) enum OpCode {
    Constant(usize),
    Return,
}

#[derive(Debug)]
pub(crate) struct Instruction {
    pub(crate) code: OpCode,
    pub(crate) line: usize,
}

// TODO: Are we able to store the constant with the instruction, rather than having an individual
// array? Would make referencing the constant much easier (to look at later down the line).
pub(crate) struct Chunk {
    pub(crate) code: Vec<Instruction>,
    pub(crate) constants: Values,
}

impl Chunk {
    pub(crate) fn init() -> Self {
        Self {
            code: Vec::new(),
            constants: Values::init(),
        }
    }

    pub(crate) fn write(&mut self, byte: OpCode, line: usize) {
        self.code.push(Instruction { code: byte, line });
    }

    pub(crate) fn add_constant(&mut self, value: Value) -> usize {
        self.constants.values.push(value);

        self.constants.values.len() - 1
    }
}
