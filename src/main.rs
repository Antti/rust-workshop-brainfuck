#[derive(Debug)]
enum Instruction {
    Increment,
    Decrement,
    Forward,
    Backward,
    JumpF,
    JumpB,
    Out,
    In,
    JumpForward(usize),
    JumpBackward(usize),
}

fn parse(input: &str) -> Vec<Instruction> {
    input.chars().flat_map(|c| {
        match c {
            '+' => Some(Instruction::Increment),
            '-' => Some(Instruction::Decrement),
            '[' => Some(Instruction::JumpF),
            ']' => Some(Instruction::JumpB),
            '>' => Some(Instruction::Forward),
            '<' => Some(Instruction::Backward),
            '.' => Some(Instruction::Out),
            ',' => Some(Instruction::In),
            _ => None
        }
    }).collect()
}

fn run(input: &str) {
    let mut memory = vec![0u8; 30_000];
    let mut pc = 0;
    let mut mc = 0;
    let mut program = parse(input);

    // println!("{:?}", program);

    while pc < program.len() {
        match program[pc] {
            Instruction::Increment => memory[mc] += 1,
            Instruction::Decrement => memory[mc] -= 1,
            Instruction::JumpForward(jump_location) => {
                if memory[mc] == 0 {
                    pc = jump_location;
                }
            },
            Instruction::JumpBackward(jump_location) => {
                if memory[mc] != 0 {
                    pc = jump_location;
                }
            },
            Instruction::Forward => mc += 1,
            Instruction::Backward => mc -= 1,
            Instruction::Out => print!("{}", memory[mc] as char),
            Instruction::In => println!("Found input"),
            Instruction::JumpF => { panic!("Unprocessed jump") }
            Instruction::JumpB => { panic!("Unprocessed jump") }
        };
        pc += 1;
    }
}

fn main() {
    run("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
}
