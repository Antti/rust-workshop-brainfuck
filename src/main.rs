#[derive(Debug, PartialEq)]
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
    let mut mp = 0;
    let mut program = parse(input);
    set_jumps(&mut program);

    while pc < program.len() {
        match program[pc] {
            Instruction::Increment => memory[mp] = (memory[mp].overflowing_add(1)).0,
            Instruction::Decrement => memory[mp] = (memory[mp].overflowing_sub(1)).0,
            Instruction::JumpForward(jump_location) => {
                if memory[mp] == 0 {
                    pc = jump_location;
                }
            },
            Instruction::JumpBackward(jump_location) => {
                if memory[mp] != 0 {
                    pc = jump_location;
                }
            },
            Instruction::Forward => mp += 1,
            Instruction::Backward => mp -= 1,
            Instruction::Out => print!("{}", memory[mp] as char),
            Instruction::In => println!("Found input"),
            Instruction::JumpF => { panic!("Unprocessed jump") }
            Instruction::JumpB => { panic!("Unprocessed jump") }
        };
        pc += 1;
    }
}

fn set_jumps(program: &mut [Instruction]) { 
    let mut stack = vec![];
    for (index, instruction) in program.iter_mut().enumerate() {
        match instruction {
            &mut Instruction::JumpF => stack.push((index, instruction)),
            &mut Instruction::JumpB => {
                match stack.pop() {
                    Some((jump_forward_index, jump_forward_instruction)) => {
                        *instruction = Instruction::JumpBackward(jump_forward_index);
                        *jump_forward_instruction = Instruction::JumpForward(index);
                    },
                    None => panic!("Mismatched brackets")
                }
            },
            _ => {}
        }
    }
}

fn main() {
    run("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
}

#[test]
fn test_set_jump() {
    let mut program = vec![Instruction::Increment,
        Instruction::JumpF,
        Instruction::Decrement,
        Instruction::JumpF,
        Instruction::JumpB,
        Instruction::JumpB
    ];

    let expected_program = vec![Instruction::Increment,
        Instruction::JumpForward(5),
        Instruction::Decrement,
        Instruction::JumpForward(4),
        Instruction::JumpBackward(3),
        Instruction::JumpBackward(1)
    ];

    set_jumps(&mut program);
    assert_eq!(program, expected_program);
}
