mod chunk;
mod debug;

use chunk::{Chunk, OpCode};

use crate::debug::disassemble_chunk;

fn main() {
    println!("Hello, world!");
    let mut chunk = Chunk::init();
    chunk.write(OpCode::OpReturn);

    disassemble_chunk(&chunk, "test chunk");

    chunk.free();
}
