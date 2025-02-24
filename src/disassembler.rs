use crate::{Chunk, OpCode};

pub(crate) struct Disassembler<'a> {
    pub(crate) chunk: &'a Chunk,
}

impl<'a> Disassembler<'a> {
    pub(crate) fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = usize::MIN;

        loop {
            if let Some(new_offset) = self.disassemble_instruction(offset) {
                if new_offset < self.chunk.code.len() {
                    offset = new_offset
                } else {
                    break;
                }
            }
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> Option<usize> {
        print!("{}", format!("{:0>4} ", offset));

        // TODO: Handle this error case (not urgent since we're just disassembling here)
        let instruction = self.chunk.code.get(offset)?;

        // This might need sorting a bit, fairly verbose...
        if offset > 0 && instruction.line == self.chunk.code.get(offset - 1)?.line {
            print!("   | ");
        } else {
            print!("{:>4} ", instruction.line);
        }

        match instruction.code {
            OpCode::Constant(constant_index) => {
                self.constant_instruction("OP_CONSTANT", &constant_index)
            }

            OpCode::Return => self.simple_instruction("OP_RETURN"),
            _ => todo!(
                "{}",
                format!("Failed to parse instruction {:?}", instruction)
            ),
        };

        return Some(offset + 1);
    }

    fn constant_instruction(&self, name: &str, constant_index: &usize) {
        print!("{:<16} {:<4}'", name, constant_index);
        self.chunk.constants.print_value(&constant_index);
        print!("'\n");
    }

    fn simple_instruction(&self, name: &str) {
        println!("{}", name);
    }
}
