use crate::chunk::{Chunk, OpCode};

/// Cranks through all the bytecode and outputs the contents of each
/// instruction in the chunk.
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

// NOTE: We return a `usize` here since it's much easier to deal with that in
// `disassembleChunk` than an i64.
/// Outputs the contents of a single instruction
fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:0>4} ", offset);

    // We need to handle the fact that `Vec<T>::get()` returns a
    // `Result<T, E>`, even though it's very unlikely we'll ever fall into the
    // error case here.
    let instruction = chunk.code.get(offset).ok_or_else(|| {
        println!(
            "Panic! Offset trying to get code out of bounds, offset: {}",
            offset
        );
        panic!();
    });

    match instruction.unwrap() {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        // NOTE: This is currently unreachable since we're just passing OpCodes
        // around - this might need to change in the future
        // _ => {
        //     println!("Unknown opcode {:?}", instruction.unwrap());
        //     return offset + 1;
        // }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);

    return offset + 1;
}
