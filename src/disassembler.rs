use crate::chunk::Instruction;
use crate::{Chunk, OpCode};

pub(crate) struct Disassembler<'a> {
    pub(crate) chunk: &'a Chunk,
}

impl<'a> Disassembler<'a> {
    #[allow(dead_code)]
    pub(crate) fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = usize::MIN;

        loop {
            if let Some(new_offset) = self.disassemble_instruction(offset) {
                if new_offset < self.chunk.instruction.len() {
                    offset = new_offset
                } else {
                    break;
                }
            }
        }
    }

    pub(crate) fn disassemble_instruction(&self, offset: usize) -> Option<usize> {
        print!("{}", format!("{:0>4} ", offset));

        // TODO: Handle this error case (not urgent since we're just disassembling here)
        let instruction = self.chunk.instruction.get(offset)?;

        // This might need sorting a bit, fairly verbose...
        if offset > 0 && instruction.line == self.chunk.instruction.get(offset - 1)?.line {
            print!("   | ");
        } else {
            print!("{:>4} ", instruction.line);
        }

        match instruction.code {
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", &instruction),

            OpCode::Add => self.simple_instruction("OP_ADD"),
            OpCode::Subtract => self.simple_instruction("OP_SUBRACT"),
            OpCode::Multiply => self.simple_instruction("OP_MULTIPLY"),
            OpCode::Divide => self.simple_instruction("OP_DIVIDE"),

            OpCode::Negate => self.simple_instruction("OP_NEGATE"),

            OpCode::Return => self.simple_instruction("OP_RETURN"),
        };

        return Some(offset + 1);
    }

    fn constant_instruction(&self, name: &str, instruction: &Instruction) {
        print!("{:<16} {:<4}'", name, 0);
        instruction
            .constants
            .clone()
            .expect("Constant instruction didn't have a constant pool initialised")
            .print_value(0);
        print!("'\n");
    }

    fn simple_instruction(&self, name: &str) {
        println!("{}", name);
    }
}
