use std::fs::{self, File};
use std::process::{Command, exit};
use std::env;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub enum IR {
    MovePointerRight(usize),
    MovePointerLeft(usize),
    Increment(usize),
    Decrement(usize),
    Output,
    Input,
    JumpForward,
    JumpBackward,
}

pub fn tokenize(code: &str) -> Vec<char> {
    code.chars()
        .filter(|c| matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect()
}

pub fn generate_ir(tokens: Vec<char>) -> Vec<IR> {
    let mut ir = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i] {
            '>' => {
                let mut step = 1;
                while i + step < tokens.len() && tokens[i + step] == '>' {
                    step += 1;
                }
                ir.push(IR::MovePointerRight(step));
                i += step;
            }
            '<' => {
                let mut step = 1;
                while i + step < tokens.len() && tokens[i + step] == '<' {
                    step += 1;
                }
                ir.push(IR::MovePointerLeft(step));
                i += step;
            }
            '+' => {
                let mut step = 1;
                while i + step < tokens.len() && tokens[i + step] == '+' {
                    step += 1;
                }
                ir.push(IR::Increment(step));
                i += step;
            }
            '-' => {
                let mut step = 1;
                while i + step < tokens.len() && tokens[i + step] == '-' {
                    step += 1;
                }
                ir.push(IR::Decrement(step));
                i += step;
            }
            '.' => {
                ir.push(IR::Output);
                i += 1;
            }
            ',' => {
                ir.push(IR::Input);
                i += 1;
            }
            '[' => {
                ir.push(IR::JumpForward);
                i += 1;
            }
            ']' => {
                ir.push(IR::JumpBackward);
                i += 1;
            }
            _ => i += 1,
        }
    }
    ir
}

pub fn generate_assembly(ir: Vec<IR>) -> String {
    let mut assembly_code = String::new();
    let mut label_counter = 0;
    let mut loop_stack: Vec<usize> = Vec::new();

    assembly_code.push_str(r#"
format elf64 executable 3
entry start

segment readable writeable
    mem rb 30000

segment executable
start:
    mov rsi, mem
"#);

    for instruction in ir {
        match instruction {
            IR::MovePointerRight(steps) => {
                assembly_code.push_str(&format!("    add rsi, {}\n", steps));
            }
            IR::MovePointerLeft(steps) => {
                assembly_code.push_str(&format!("    sub rsi, {}\n", steps));
            }
            IR::Increment(inc) => {
                assembly_code.push_str(&format!("    add byte [rsi], {}\n", inc));
            }
            IR::Decrement(dec) => {
                assembly_code.push_str(&format!("    sub byte [rsi], {}\n", dec));
            }
            IR::Output => {
                assembly_code.push_str(r#"
    mov rax, 1
    mov rdi, 1
    mov rdx, 1
    ; rsi already points to buffer
    syscall
"#);
            }
            IR::Input => {
                assembly_code.push_str(r#"
    mov rax, 0
    mov rdi, 0
    mov rdx, 1
    ; rsi already points to buffer
    syscall
"#);
            }
            IR::JumpForward => {
                let label = label_counter;
                loop_stack.push(label);
                assembly_code.push_str(&format!(
                    "    cmp byte [rsi], 0\n    je loop_end_{}\n    loop_start_{}:\n",
                    label, label
                ));
                label_counter += 1;
            }
            IR::JumpBackward => {
                if let Some(label) = loop_stack.pop() {
                    assembly_code.push_str(&format!(
                        "    cmp byte [rsi], 0\n    jne loop_start_{}\n    loop_end_{}:\n",
                        label, label
                    ));
                }
            }
        }
    }

    assembly_code.push_str(r#"
    mov rax, 60
    xor rdi, rdi
    syscall
"#);

    assembly_code
}

pub fn write_assembly_to_file(assembly_code: &str, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(assembly_code.as_bytes())?;
    Ok(())
}

pub fn assemble_and_link() -> io::Result<()> {
    // Assemble using FASM
    let output = Command::new("fasm")
        .arg("output.asm")
        .output()?;

    if !output.status.success() {
        eprintln!("Error assembling with FASM:\n{}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    // Rename from default FASM output to desired program name
    fs::rename("output", "brainfuck_program")?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <brainfuck_file.bf>", args[0]);
        exit(1);
    }

    let file_path = &args[1];

    let mut brainfuck_code = String::new();
    File::open(file_path)?.read_to_string(&mut brainfuck_code)?;

    let tokens = tokenize(&brainfuck_code);
    let ir = generate_ir(tokens);
    let assembly_code = generate_assembly(ir);
    write_assembly_to_file(&assembly_code, "output.asm")?;
    assemble_and_link()?;

    println!("Successfully compiled '{}' into 'brainfuck_program'", file_path);

    Ok(())
}
