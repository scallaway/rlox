use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
}

impl VM {
    pub fn init() {}

    pub fn free() {}

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = self.chunk.code.len();

        return self.run();
    }

    fn read_byte(&self, ip: usize) -> &OpCode {
        self.chunk.code.get(ip).unwrap_or_else(|| {
            println!(
                "\nERROR: Couldn't read instruction from Chunk at IP: {}",
                self.ip
            );
            panic!();
        })
    }

    fn read_constant(&self, constant: &usize) -> &Value {
        &self
            .chunk
            .constants
            .values
            .get(*constant)
            .unwrap_or_else(|| {
                println!(
                    "\nERROR: Couldn't read constant from Chunk at IP: {}",
                    self.ip
                );
                panic!();
            })
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_byte(self.ip);
            self.ip += 1;

            return match instruction {
                OpCode::OpConstant(constant) => {
                    let constant = self.read_constant(constant);
                    Value::print(constant);
                    println!("");

                    // TODO: Change this in the future
                    return InterpretResult::Ok;
                }
                OpCode::OpReturn => InterpretResult::Ok,
            };
        }
    }
}
