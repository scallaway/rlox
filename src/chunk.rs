use crate::value::Values;

// TODO: impl fmt::Display
#[derive(Debug)]
pub(crate) enum OpCode {
    // I have a feeling this might need to take an index after all, but we'll see further down the
    // line
    Constant,
    Negate,
    Return,
}

#[derive(Debug)]
pub(crate) struct Instruction {
    pub(crate) code: OpCode,
    pub(crate) line: usize,
    pub(crate) constants: Option<Values>,
}

pub(crate) struct Chunk {
    pub(crate) instruction: Vec<Instruction>,
}

impl Chunk {
    pub(crate) fn init() -> Self {
        Self {
            instruction: Vec::new(),
        }
    }

    pub(crate) fn write(&mut self, byte: OpCode, line: usize, constants: Option<Values>) {
        self.instruction.push(Instruction {
            code: byte,
            line,
            constants,
        });
    }
}
