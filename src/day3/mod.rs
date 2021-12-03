/// Gets a char at position `n` in `line`
fn get_nth_char(line: &String, n: usize) -> char {
    line.chars().nth(n).expect(&format!("line has no index {}", n))
}

/// Iterates over all rows and counts how often `character` occurs at `column`
fn count_occurrences_in_column(rows: &Vec<&String>, column: usize, character: char) -> u64{
    rows.iter().fold(0, |p, line| p + ((get_nth_char(line, column) == character) as u64))
}

/// Counts counts the occurences of a specific character (here literally 0 and 1) in a column
/// of a matrix (technically a vector of String). Depending on which number occurs more often
/// it will generate another (Bit-)String which is interpreted as an actual number
pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 3 task 1");

    // Just create a new Vector which holds references to the data of the input data
    let lines : Vec<&String> = data.iter().map(|line| line).collect();

    // Count how many columns we have (i.e., the length of the Strings)
    let n_cols = data.get(0).expect("No data glines given").len();

    // Holds the Bit-Strings which are to be constructed
    let mut gamma = String::new();
    let mut epsilon = String::new();

    // iterate over each column
    for i in 0..n_cols {
        // count ones and zeros over each row for column `i`
        let n_ones = count_occurrences_in_column(&lines, i,'1');
        let n_zeros = count_occurrences_in_column(&lines, i,'0');

        // Construct the bit string as per task
        if n_ones > n_zeros {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    // Auto Magic to transform a bit string to an actual number
    let gamma_number = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_number = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("gamma {} ({}), epsilon {} ({})", gamma, gamma_number, epsilon, epsilon_number);

    // ... and return their products
    return (gamma_number * epsilon_number) as u64;
}

/// This one actually consecutively filters the rows depending on a rule which acts on the remaining
/// rows. This is done until there is only one line left which is then interpreted as bit string
/// (as in task 1)
pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 3 task 2");

    // count columns and create 2 Vectors holding references (views) to the input lines
    // (i.e., we do NOT copy the data itself but generate two Vectors referencing the original data)
    let n_cols = data.get(0).expect("No data lines given").len();
    let mut oxygen : Vec<&String> = data.iter().map(|line| line).collect();
    let mut scrubber : Vec<&String> = data.iter().map(|line| line).collect();

    // iterate over each column
    for i in 0..n_cols {

        // check if have more than one line
        if oxygen.len() > 1 {

            // count ones and zeros
            let n_ones = count_occurrences_in_column(&oxygen, i,'1');
            let n_zeros = count_occurrences_in_column(&oxygen, i,'0');

            // ... filter as per rules given
            oxygen.retain(|line| {
                let char = get_nth_char(line, i);
                if n_ones >= n_zeros {
                    return char == '1';
                }
                return char != '1';
            });
        }

        // do basically the same as above with different rules
        // this could be optimzed, though as it violates DRY-Principles (dont repeat yourself)
        // we could fix this by adding a filter function which consumes a filter rule and the data
        // where just the filter rule is replaced
        if scrubber.len() > 1 {
            let n_ones = count_occurrences_in_column(&scrubber, i,'1');
            let n_zeros = count_occurrences_in_column(&scrubber, i,'0');

            scrubber.retain(|line| {
                let char = get_nth_char(line, i);
                if n_ones < n_zeros {
                    return char == '1';
                }
                return char != '1';
            });
        }
    }

    let oxygen_number = isize::from_str_radix(
        oxygen.get(0).unwrap(),
        2
    ).unwrap();
    let scrubber_number = isize::from_str_radix(
        scrubber.get(0).unwrap(), 2
    ).unwrap();

    println!("oygen: {:?} ({}), scrubber: {:?} ({})", oxygen, oxygen_number, scrubber, scrubber_number);

    return (oxygen_number * scrubber_number) as u64;

}