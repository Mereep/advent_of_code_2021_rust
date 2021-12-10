use crate::day4::parse_line_of_numbers;

/// List of sorted open brackets
const OPEN_BRACKETS: [char; 4] =    ['(', '{', '<', '['];
/// List of sorted closing brackets (relative to OPEN_BRACKETS)
const CLOSING_BRACKETS: [char; 4] = [')', '}', '>', ']'];

pub fn index_of<T: std::cmp::PartialEq + Clone>(haystack: &[T], needle: T) -> Option<usize>{
    haystack.iter().position(|i| *i == needle)
}

/// map the brackets to error counts for task 1
pub fn syntax_error_to_points_task_1(error: char) -> u64 {
    return match error {
        '}' => 1197,
        ')' => 3,
        '>' => 25137,
        ']' => 57,
        _ => panic!("Unknown syntax error")
    }
}

/// map the brackets to error counts for task 2
pub fn syntax_error_to_points_task_2(error: char) -> u64 {
    return match error {
        '}' => 3,
        ')' => 1,
        '>' => 4,
        ']' => 2,
        _ => panic!("Unknown syntax error")
    }
}

/// Returns the first character to be found which is a syntax error
/// if no syntax error is found, `None` is returned
pub fn find_illegal_character(data: &String) -> Option<char> {

        // we store all found brackets in a stack
        let mut stack: Vec<char> = Vec::new();
        for char in data.chars() {
            // if we find a open bracket we increase the stack by the open bracket
            if OPEN_BRACKETS.contains(&char) {
                stack.push(char)
            } else {
                // if we find a closing bracket we pop the highest element from the stack
                // this is the expected (counterpart-)bracket to find
                let expected_pair = stack.pop().unwrap();

                // we get the position of the expected bracket in our OPEN_BRACKETS array
                let expected_pair_pos = index_of(
                    &OPEN_BRACKETS,
                      expected_pair).unwrap();

                // and if it doesn't match the expected bracket we search we return the bracket
                if CLOSING_BRACKETS[expected_pair_pos] != char {
                    return Some(char);
                }
            }
        }

    return None
}

/// Returns the expected closing brackets in expectation order
/// (reversed stack)
///
/// This works like the function above except it doesn't expect any syntax errors
pub fn return_expected_closing_brackets(data: &String) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    for char in data.chars() {
        if OPEN_BRACKETS.contains(&char) {
            stack.push(char)
        } else {
            let expected_pair = stack.pop().unwrap();
            let expected_pair_pos = index_of(
                &OPEN_BRACKETS,
                  expected_pair).unwrap();
            // We do NOT expect errors here anymore (they have to be filtered before)
            if CLOSING_BRACKETS[expected_pair_pos] != char {
                panic!("Syntax error was not expected here")
            }
        }
    }

    stack.reverse();
    return stack.iter().map(
        |c|
        CLOSING_BRACKETS[index_of(&OPEN_BRACKETS, *c).unwrap()]).collect();
}

pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 10 task 1");

    // sum the errors for each line
    let mut error_sum = 0 as u64;
    for line in data {
        if let Some(error) = find_illegal_character(line) {
            error_sum += syntax_error_to_points_task_2(error);
        }
        // println!("{:?}", find_illegal_character(line));
    }
    return error_sum;
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 10 task 2");

    // filter out all lines that have errors (we do not care for them)
    let incomplete_lines= data
        .iter()
        .filter(|line| find_illegal_character(line).is_none()).
        collect::<Vec<&String>>();

    // Store each error score as a member in this array
    let mut errors: Vec<u64> = Vec::new();
    for line in incomplete_lines {
        let expected_brackets = return_expected_closing_brackets(line);
        let sum = expected_brackets
            .iter()
            .fold(0 as u64, |p, c| p * 5 + syntax_error_to_points_task_2(*c));
        errors.push(sum);
    }

    // sort the array
    errors.sort();

    // ... and return the median element
    return *errors.get(errors.len() / 2).unwrap();
}
