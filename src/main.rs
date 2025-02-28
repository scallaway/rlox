use chunk::{Chunk, OpCode};
use value::{Value, Values};

mod chunk;
mod disassembler;
mod value;
mod vm;

fn main() {
    let mut vm = vm::VM::init();
    let mut chunk = Chunk::init();

    chunk.write(
        OpCode::Constant,
        123,
        Some(Values {
            values: vec![Value(1.2)],
        }),
    );

    chunk.write(OpCode::Negate, 123, None);
    chunk.write(OpCode::Return, 123, None);

    let _ = vm.interpret(&chunk);
}
