mod chunk;
mod debug;
mod value;

use chunk::{Chunk, OpCode};
use value::Value;

use crate::debug::disassemble_chunk;

fn main() {
    let mut chunk = Chunk::init();
    let constant = chunk.add_constant(Value(1.2));
    chunk.write(OpCode::OpConstant(constant), 123);
    chunk.write(OpCode::OpReturn, 123);

    disassemble_chunk(&chunk, "test chunk");

    chunk.free();
}
