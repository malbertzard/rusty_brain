use std::process::{Command, exit};

pub fn generate_assembly(ir: Vec<super::ir::IR>) -> String {
    let mut assembly_code = String::new();

    assembly_code.push_str(r#"
section .data
    mem resb 30000  ; reserve 30,000 bytes for Brainfuck memory
section .bss
    pointer resq 1  ; the pointer will be an integer in rsi (the memory index)
section .text
    global _start
_start:
    ; Initialize the pointer (memory index)
    mov rsi, mem  ; Set pointer to the base of the memory
"#);

    for instruction in ir {
        match instruction {
            super::ir::IR::MovePointerRight(steps) => {
                for _ in 0..steps {
                    assembly_code.push_str("    add rsi, 1\n");
                }
            }
            super::ir::IR::MovePointerLeft(steps) => {
                for _ in 0..steps {
                    assembly_code.push_str("    sub rsi, 1\n");
                }
            }
            super::ir::IR::Increment(inc) => {
                for _ in 0..inc {
                    assembly_code.push_str("    inc byte [rsi]\n");
                }
            }
            super::ir::IR::Decrement(dec) => {
                for _ in 0..dec {
                    assembly_code.push_str("    dec byte [rsi]\n");
                }
            }
            super::ir::IR::Output => {
                assembly_code.push_str(r#"
    ; Output the value at the pointer
    mov al, [rsi]   ; Move the byte at pointer to al register
    mov rdi, 1      ; File descriptor 1 (stdout)
    mov rsi, rdi    ; The character to print
    mov rdx, 1      ; Size of the data
    syscall         ; invoke system call
"#);
            }
            super::ir::IR::Input => {
                assembly_code.push_str(r#"
    ; Input a single character and store it at the pointer
    mov rdi, 0      ; File descriptor 0 (stdin)
    mov rsi, rsi    ; Pointer where to store the input
    mov rdx, 1      ; Size of the input
    syscall         ; invoke system call
"#);
            }
            super::ir::IR::JumpForward => {
                assembly_code.push_str(r#"
    ; Jump forward: if the byte at rsi is zero, jump to the matching ']'
    cmp byte [rsi], 0
    je .jump_forward
"#);
            }
            super::ir::IR::JumpBackward => {
                assembly_code.push_str(r#"
    ; Jump backward: if the byte at rsi is not zero, jump to the matching '['
    cmp byte [rsi], 0
    jne .jump_backward
"#);
            }
        }
    }

    assembly_code.push_str(r#"
    ; Exit the program
    mov rax, 60         ; syscall number for exit
    xor rdi, rdi        ; status 0
    syscall
"#);

    assembly_code
}

pub fn assemble_and_link() -> std::io::Result<()> {
    // Assemble the code using nasm
    let output = Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg("output.o")
        .arg("output.asm")
        .output()?;

    if !output.status.success() {
        eprintln!("Error assembling the code");
        exit(1);
    }

    let output = Command::new("ld")
        .arg("-o")
        .arg("brainfuck_program")
        .arg("output.o")
        .output()?;

    if !output.status.success() {
        eprintln!("Error linking the object file");
        exit(1);
    }

    Ok(())
}
