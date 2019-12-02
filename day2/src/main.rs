use std::env::args;
use std::fs::File;
use std::io::{Read, Result};

type OpCode = usize;

fn execute(opcodes: &mut Vec<OpCode>) -> Vec<OpCode> {
    let mut i = 0;
    while i < opcodes.len() {
        match opcodes[i] {
            1 => {
                let target_idx = opcodes[i + 3];
                let left_idx = opcodes[i + 1];
                let right_idx = opcodes[i + 2];
                opcodes[target_idx] = opcodes[left_idx] + opcodes[right_idx];
                i += 4;
            }
            2 => {
                let target_idx = opcodes[i + 3];
                let left_idx = opcodes[i + 1];
                let right_idx = opcodes[i + 2];
                opcodes[target_idx] = opcodes[left_idx] * opcodes[right_idx];
                i += 4;
            }
            99 => {
                break;
            }
            _ => panic!("Invalid opcode, first value is {}", opcodes[0]),
        }
    }
    opcodes.clone()
}

fn main() -> Result<()> {
    let file_path = args().nth(1).expect("Missing argument");
    println!("Reading from file {}", file_path);

    let output = execute(&mut vec![1, 0, 0, 0, 99]);
    assert_eq!(vec![2, 0, 0, 0, 99], output);
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // println!("{}", buffer);
    buffer = buffer.trim().to_owned();
    // for i in buffer.split(",") {
    //     println!("{}({})", i, i.len());
    //     println!("parsed :{:?}", i.parse::<OpCode>().unwrap());
    // }
    let mut program: Vec<OpCode> = buffer
        .split(",")
        .map(|n| n.parse::<OpCode>().unwrap())
        .collect();
    println!("Configuring program for crash reproduction...");
    program[1] = 12;
    program[2] = 2;

    println!("Starting execution...");
    let result = execute(&mut program);
    println!("{}", result[0]);

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
