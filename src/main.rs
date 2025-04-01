mod tokenizer;
mod ir;
mod assembler;
mod utils;

use tokenizer::tokenize;
use ir::generate_ir;
use assembler::{generate_assembly, assemble_and_link};
use utils::write_assembly_to_file;

fn main() -> std::io::Result<()> {
    let brainfuck_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    // Tokenize the Brainfuck code
    let tokens = tokenize(brainfuck_code);

    // Convert tokens to IR
    let ir = generate_ir(tokens);

    // Generate assembly from IR
    let assembly_code = generate_assembly(ir);

    // Write the assembly code to a file
    write_assembly_to_file(&assembly_code, "output.asm")?;

    // Assemble and link the assembly code into an executable binary
    assemble_and_link()?;

    println!("Successfully compiled Brainfuck code into 'brainfuck_program'");

    Ok(())
}
