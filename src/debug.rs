use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

/// Cranks through all the bytecode and outputs the contents of each
/// instruction in the chunk.
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    for offset in 0..chunk.code.len() {
        disassemble_instruction(chunk, offset);
    }
}

// NOTE: We return a `usize` here since it's much easier to deal with that in
// `disassembleChunk` than an i64.
/// Outputs the contents of a single instruction
fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    print!("{:0>4} ", offset);

    if offset > 0 && chunk.lines.get(&offset).unwrap() == chunk.lines.get(&(offset - 1)).unwrap() {
        print!("   | ");
    } else {
        print!("{:>4} ", chunk.lines.get(&offset).unwrap());
    }

    // We need to handle the fact that `Vec<T>::get()` returns a
    // `Result<T, E>`, even though it's very unlikely we'll ever fall into the
    // error case here.
    let instruction = chunk.code.get(offset).unwrap_or_else(|| {
        println!(
            "Panic! Offset trying to get code out of bounds, offset: {}",
            offset
        );
        panic!();
    });

    match instruction {
        OpCode::OpConstant(constant) => constant_instruction("OP_CONSTANT", chunk, *constant),
        OpCode::OpReturn => simple_instruction("OP_RETURN"),
        // NOTE: This is currently unreachable since we're just passing OpCodes
        // around - this might need to change in the future
        // _ => {
        //     println!("Unknown opcode {:?}", instruction.unwrap());
        //     return offset + 1;
        // }
    };
}

fn constant_instruction(name: &str, chunk: &Chunk, constant: usize) {
    print!(
        "{} {:>4} '",
        format!("{:width$}", name, width = 16),
        constant
    );

    let value = chunk.constants.values.get(constant).unwrap_or_else(|| {
        println!("ERROR: There was a problem retrieving the constant for debugging");
        panic!();
    });

    Value::print(value);

    print!("'\n");
}

fn simple_instruction(name: &str) {
    println!("{}", name);
}
