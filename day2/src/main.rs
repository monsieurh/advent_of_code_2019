use std::env::args;
use std::fs::File;
use std::io::{Read, Result};

type OpCode = usize;

fn execute(opcodes: &mut Vec<OpCode>) -> Vec<OpCode> {
    let mut instruction_ptr = 0;
    while instruction_ptr < opcodes.len() {
        match opcodes[instruction_ptr] {
            1 => {
                let target_idx = opcodes[instruction_ptr + 3];
                let left_idx = opcodes[instruction_ptr + 1];
                let right_idx = opcodes[instruction_ptr + 2];
                opcodes[target_idx] = opcodes[left_idx] + opcodes[right_idx];
                instruction_ptr += 4;
            }
            2 => {
                let target_idx = opcodes[instruction_ptr + 3];
                let left_idx = opcodes[instruction_ptr + 1];
                let right_idx = opcodes[instruction_ptr + 2];
                opcodes[target_idx] = opcodes[left_idx] * opcodes[right_idx];
                instruction_ptr += 4;
            }
            99 => {
                instruction_ptr += 1;
                break;
            }
            _ => panic!("Invalid opcode, first value is {}", opcodes[0]),
        }
    }
    opcodes.clone()
}

fn run_opcode_program(memory: &mut Vec<OpCode>, noun: OpCode, verb: OpCode) -> usize {
    println!("Starting program execution [noun:{}, verb:{}]", noun, verb);
    memory[1] = noun;
    memory[2] = verb;

    let result = execute(memory);
    result[0]
}

fn main() -> Result<()> {
    println!("Booting flight computer...");
    let file_path = args().nth(1).expect("Missing argument");

    println!("Reading memory file '{}'", file_path);
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    buffer = buffer.trim().to_owned();

    let memory: Vec<OpCode> = buffer
        .split(",")
        .map(|n| n.parse::<OpCode>().unwrap())
        .collect();
    println!("Memory file loaded : {} opcodes", memory.len());

    let result = run_opcode_program(&mut memory.clone(), 12, 2);

    println!("Program output (memory @0x0000):{}", result);
    Ok(())
}

mod tests {
    use super::execute;

    #[test]
    fn test_program1() {
        let output = execute(&mut vec![1, 0, 0, 0, 99]);
        assert_eq!(vec![2, 0, 0, 0, 99], output);
    }
    #[test]
    fn test_program2() {
        let output = execute(&mut vec![2, 3, 0, 3, 99]);
        assert_eq!(vec![2, 3, 0, 6, 99], output);
    }
    #[test]
    fn test_program3() {
        let output = execute(&mut vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], output);
    }
    #[test]
    fn test_program4() {
        let output = execute(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], output);
    }
}
