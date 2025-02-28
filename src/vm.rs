use crate::{
    chunk::{Chunk, Instruction},
    Disassembler, OpCode, Value,
};

pub(crate) struct VM<'a> {
    pub chunk: Option<&'a Chunk>,
    pub ip: usize,
    pub stack: Vec<Value>,
}

pub(crate) enum InterpretError {
    Compile,
    Runtime,
}

impl<'a> VM<'a> {
    pub(crate) fn init() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub(crate) fn interpret(&mut self, chunk: &'a Chunk) -> Result<(), InterpretError> {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            let instruction = self.read_byte();

            if cfg!(debug_assertions) {
                print!("          ");
                for slot in self.stack.iter() {
                    print!("[ ");
                    slot.print();
                    print!(" ]");
                }
                print!("\n");

                let disassembler = Disassembler {
                    chunk: self
                        .chunk
                        .expect("Didn't have a chunk to pass to the disassembler"),
                };
                // NOTE: The self.ip here might be wrong
                disassembler.disassemble_instruction(self.ip.clone());
            }

            match instruction.code {
                OpCode::Constant => {
                    // This could be improved by not using a Vec I think
                    let constant = instruction
                        .constants
                        .clone()
                        .expect("Constant pool for OpCode::Constant not initialised")
                        .values
                        .get(0)
                        .expect("Failed to find constant for OpCode::Constant")
                        .clone();

                    self.stack.push(constant.clone());
                }

                OpCode::Negate => {
                    let value = self
                        .stack
                        .pop()
                        .expect("Failed to pop value for OpCode::Negate")
                        .0;
                    self.stack.push(Value(-value));
                }

                OpCode::Return => {
                    let value = self.stack.pop().expect("Failed to pop value off the stack");
                    value.print();
                    print!("\n");

                    return Ok(());
                }
            };

            self.ip += 1;
        }
    }

    fn read_byte(&self) -> &Instruction {
        let instruction = self
            .chunk
            .unwrap()
            .instruction
            .get(self.ip)
            .expect(&format!("Failed to get code at IP position: {:?}", self.ip));

        // self.ip += 1;

        &instruction
    }
}
