use crate::{Chunk, OpCode};

pub(crate) struct Disassembler<'a> {
    pub(crate) chunk: &'a Chunk,
}

impl<'a> Disassembler<'a> {
    pub(crate) fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = usize::MIN;

        while offset < self.chunk.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{}", format!("{:0<4} ", offset));

        let instruction = self
            .chunk
            .code
            .get(offset)
            .expect(format!("Failed to get instruction at offset {}", offset).as_str());

        match instruction {
            OpCode::Constant(_) => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::Return => self.simple_instruction("OP_RETURN", offset),
            _ => todo!(
                "{}",
                format!("Failed to parse instruction {:?}", instruction)
            ),
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant_opcode = self
            .chunk
            .code
            .get(offset + 1)
            .expect(format!("Failed to get code at offset: {}", offset + 1).as_str());

        match constant_opcode {
            OpCode::Constant(index) => {
                print!("{:0<16} {:0<4}", name, index);
                self.chunk.constants.print_value(index);
                print!("\n");

                offset + 2
            }
            _ => panic!(
                "{}",
                format!("Fetched incorrect OpCode at index: {}", offset + 1)
            ),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);

        offset + 1
    }
}
