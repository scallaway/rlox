use crate::{
    chunk::{Chunk, Instruction},
    disassembler::Disassembler,
    OpCode, Value,
};

pub(crate) struct VM<'a> {
    pub chunk: Option<&'a Chunk>,
    pub ip: usize,
    pub stack: Vec<Value>,
}

#[allow(dead_code)]
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

            let code = self.read_byte().code;

            match code {
                OpCode::Constant => {
                    // This could be improved by not using a Vec I think
                    let constant = self
                        .read_byte()
                        .constants
                        .clone()
                        .expect("Constant pool for OpCode::Constant not initialised")
                        .values
                        .get(0)
                        .expect("Failed to find constant for OpCode::Constant")
                        .clone();

                    self.stack.push(constant.clone());
                }

                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    self.binary_op(&code)
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
                #[allow(unreachable_patterns)]
                _ => {
                    return Err(InterpretError::Runtime);
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

        &instruction
    }

    fn binary_op(&mut self, code: &OpCode) {
        // Verify we have enough values on the stack to execute the operation
        assert!(self.stack.len() >= 2);

        let b = self.stack.pop().unwrap().0;
        let a = self.stack.pop().unwrap().0;

        match code {
            OpCode::Add => self.stack.push(Value(a + b)),
            OpCode::Subtract => self.stack.push(Value(a - b)),
            OpCode::Multiply => self.stack.push(Value(a * b)),
            OpCode::Divide => self.stack.push(Value(a / b)),
            _ => panic!(
                "{}",
                format!(
                    "Tried to execute a Binary Operation without the correct OpCode: {:?}",
                    code
                )
            ),
        };
    }
}
