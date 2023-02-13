mod chunk;
mod debug;
mod value;
mod vm;

use chunk::{Chunk, OpCode};
use value::Value;
use vm::VM;

use crate::debug::disassemble_chunk;

fn main() {
    VM::init();

    let mut chunk = Chunk::init();
    let constant = chunk.add_constant(Value(1.2));
    chunk.write(OpCode::OpConstant(constant), 123);
    chunk.write(OpCode::OpReturn, 123);

    disassemble_chunk(&chunk, "test chunk");

    VM::free();
    chunk.free();
}
