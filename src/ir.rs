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
