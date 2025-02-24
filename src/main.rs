use chunk::{Chunk, OpCode};
use disassembler::Disassembler;

mod chunk;
mod disassembler;
mod value;

fn main() {
    let mut chunk = Chunk::init();

    let constant_index = chunk.add_constant(value::Value(1.2));
    chunk.write(OpCode::Constant(constant_index), 123);

    chunk.write(OpCode::Return, 123);

    let disassembler = Disassembler { chunk: &chunk };
    disassembler.disassemble("test chunk");
}
