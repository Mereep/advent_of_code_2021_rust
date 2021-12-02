
///We only have 3 commands (we cannot move backwards)
#[derive(Debug)]
enum Command {
    Forward,
    Down,
    Up
}

/// An instruction consist of a specific `Command` and a value
#[derive(Debug)]
struct Instruction {
    command: Command,
    value: u64
}

/// Will parse a single line from the input file to a `Command` and its value
fn to_instruction(line: &str) -> Result<Instruction, String> {

    // We split on a whitespace
    let split: Vec<&str> = line.split(' ').collect();

    // ... and expect to get exactly two parts out
    if split.len() != 2 {
        return Err(format!("Couldn't parse line {}", line));
    }

    // retrieve the single two parts (as string)
    let command_str = *split.get(0).unwrap();
    let value_str = *split.get(1).unwrap();

    // Check if we understand the command
    return if let Some(command) = match command_str {
        "forward" => Some(Command::Forward),
        "down" => Some(Command::Down),
        "up" => Some(Command::Up),
        _ => None
    } {
        // .. and if so try to parse the number
        if let Ok(value) = value_str.parse::<u64>() {
            Ok(Instruction { command, value })
        } else {
            Err(format!("Couldn't parse value {}", value_str))
        }
    } else {
        Err(format!("Couldn't understand command {}", command_str))
    }

}

/// Just a wrapper calling [to_instruction] on every line given
fn to_instructions(data: &Vec<String>) -> Vec<Instruction>{
    return data.iter().map(|line| {
        match to_instruction(line){
            Ok(instruction) => instruction,
            Err(msg) => panic!("{}", msg)
        }
    }).collect();
}

/// Will do move a submarine
/// - parse each line into a [Instruction]
/// - execute the instruction (horizontal movement and depth movement)
///
/// he redult is the product of depth and horizontal pos
pub fn task1(data: &Vec<String>) -> u64 {
    let instructions = to_instructions(data);

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
        }
    }


    return horizontal_pos * depth;
}

/// This task is basically like the first but adding an additional `aim` (like a movement vector)
pub fn task2(data: &Vec<String>) -> u64 {
    let instructions = to_instructions(data);

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
        }
    }


    return horizontal_pos * depth;
}
