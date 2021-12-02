#[derive(Debug)]
enum Command {
    Forward,
    Down,
    Backward,
    Up
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    value: u64
}

fn to_instruction(line: &str) -> Result<Instruction, String> {
    let split: Vec<&str> = line.split(' ').collect();
    if split.len() != 2 {
        return Err(format!("Couldn't parse line {}", line));
    }

    let command_str = *split.get(0).unwrap();
    let value_str = *split.get(1).unwrap();

    return if let Some(command) = match command_str {
        "forward" => Some(Command::Forward),
        "down" => Some(Command::Down),
        "up" => Some(Command::Up),
        "backward" => Some(Command::Backward),
        _ => None
    } {
        if let Ok(value) = value_str.parse::<u64>() {
            Ok(Instruction { command, value })
        } else {
            Err(format!("Couldn't parse value {}", value_str))
        }
    } else {
        Err(format!("Couldn't understand command {}", command_str))
    }

}

pub fn task1(data: &Vec<String>) -> u64 {
    let instructions : Vec<Instruction> = data.iter().map(|line| {
        match  to_instruction(line){
            Ok(instruction) => instruction,
            Err(msg) => panic!("{}", msg)
        }
    }).collect();

    let mut horizontal_pos = 0;
    let mut depth = 0;

    for instruction in &instructions {
        match instruction {
            Instruction {command: Command::Up, value} => {
                depth -= *value;
            },
            Instruction {command: Command::Down, value} => {
                depth += *value;
            },
            Instruction {command: Command::Forward, value} => {
                horizontal_pos += *value;
            },
            _ => {
                println!("Cannot process Instruction {:?}", instruction);
            }
        }
    }


    return horizontal_pos * depth;
}

pub fn task2(data: &Vec<String>) -> u64 {
    let instructions : Vec<Instruction> = data.iter().map(|line| {
        match  to_instruction(line){
            Ok(instruction) => instruction,
            Err(msg) => panic!("{}", msg)
        }
    }).collect();

    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim: u64 = 0;

    for instruction in &instructions {
        match instruction {
            Instruction {command: Command::Up, value} => {
                aim -= value;
            },
            Instruction {command: Command::Down, value} => {
                aim += value;
            },
            Instruction {command: Command::Forward, value} => {
                horizontal_pos += value;
                depth += aim * value;
            },
            _ => {
                println!("Cannot process Instruction {:?}", instruction);
            }
        }
    }


    return horizontal_pos * depth;
}
