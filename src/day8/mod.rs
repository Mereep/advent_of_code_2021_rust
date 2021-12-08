use std::collections::{HashSet};

/// Used Layout
///  1111
/// 0    2
/// 0    2
///  3333
/// 4    6
/// 4    6
///  5555
///
#[derive(Debug)]
struct SegmentClock {
    input_line: SegmentLine,
    /// The index is like given above.
    /// This stores the segment name for a given segment index
    segments_to_letters: [Option<char>; 7]
}

impl SegmentClock {

    /// Parse an input sequence line as in `adc` (meaning having those segments turned on)
    /// to its corresponding digit
    pub fn to_digit(&self, line: &String) -> Result<u8, ()>{
        // Iterate over all patterns
        for (res, expected_segments) in [
            vec!(0, 1, 2, 4, 5, 6), // 0
            vec!(2, 6), // 1
            vec!(1, 2, 3, 4, 5), // 2
            vec!(1, 2, 3, 5 ,6), // 3
            vec!(0, 2, 3, 6), // 4
            vec!(0, 1, 3, 5, 6), // 5
            vec!(0, 1, 3, 4, 5, 6), // 6
            vec!(1, 2, 6), // 7
            vec!(0, 1, 2, 3, 4, 5, 6), // 8
            vec!(0, 1, 2, 3, 5, 6), // 9
            // check 0
        ].iter().enumerate() {
            // Filter all digits which are turned on
            let filtered = expected_segments.iter().filter(|i| line.contains(self.segments_to_letters[**i as usize].unwrap())).collect::<Vec<_>>();

            // And check if all digits are in the input sequence
            if line.len() == expected_segments.len() && filtered.len() == expected_segments.len() {
                return Ok(res as u8);
            }
        }

        return Err(());
    }

    fn output_to_number(&self) -> u64 {
        let mut whole_number = String::new();

        for digit in self.input_line.output_part.iter() {
            whole_number.push_str(&self.to_digit(digit).unwrap().to_string());
        }

        return whole_number.parse().unwrap();
    }

    pub fn from_segment_line(line: SegmentLine) -> Self {
        return Self {
            input_line: line,
            segments_to_letters: [None; 7]
        }
    }

    pub fn deduce_segments(&mut self) {

        // Char one can be directly read off
        let one = filter_by_length(&self.input_line.input_part, 2).first().unwrap().to_string();

        // 7, 8, 4 also
        let seven = filter_by_length(&self.input_line.input_part, 3).first().unwrap().to_string();
        let four = filter_by_length(&self.input_line.input_part, 4).first().unwrap().to_string();
        let eight = filter_by_length(&self.input_line.input_part, 7).first().unwrap().to_string();

        // These ones are not unique: 5,2,3 do all have the same length
        let two_and_five_and_three = filter_by_length(&self.input_line.input_part, 5);
        //... although we do not know which one is which, we know those are exactly 3 elements
        let two_and_five_element_0 = string_to_char_set(&two_and_five_and_three.first().unwrap().to_string());
        let two_and_five_element_1 = string_to_char_set(&two_and_five_and_three.get(1).unwrap().to_string());
        let two_and_five_element_2 = string_to_char_set(&two_and_five_and_three.get(2).unwrap().to_string());

        // parse the digits of one and seven as char
        let one_char: HashSet<char> = string_to_char_set(&one);
        let seven_chars: HashSet<char> = string_to_char_set(&seven);

        // ...this lets us deduce the first segment directly by diffing the 7 with the 1 (there is only one segment left)
        let segment_1 = *seven_chars.difference(&one_char).collect::<Vec<&char>>().first().unwrap().clone();

        // by intersecting (set AND) we overlap the digits one 3 and 5 which leaves os with segments 1,3,5
        let segments_1_3_5 = two_and_five_element_0.intersection(&two_and_five_element_1).cloned().collect::<HashSet<char>>().intersection(&two_and_five_element_2).cloned().collect();

        // by removing the the known segment one we have segments 3 and 5 left
        let segments_3_5 = sub_char_from_hashset(&segments_1_3_5, segment_1);

        // .. and by removing the segments of the digit 4 we we have segment 5 left
        let segment_5 = *segments_3_5.difference(&string_to_char_set(&four)).collect::<Vec<&char>>().first().unwrap().clone();
        // and we use the known segment 5 we have segment 3 left
        let segment_3 = *sub_char_from_hashset(&segments_3_5, segment_5).iter().collect::<Vec<&char>>().first().unwrap().clone();

        // ... and so on ;)
        let four_minus_one: HashSet<char> = string_to_char_set(&four)
            .difference(&string_to_char_set(&one))
            .map(|c| c.clone()).collect();
        let segment_0 = *sub_char_from_hashset(&four_minus_one.clone(), segment_3).iter().collect::<Vec<&char>>().first().unwrap().clone();
        let eight_minus_four: HashSet<char> = string_to_char_set(&eight)
            .difference(&string_to_char_set(&four))
            .map(|c|c.clone())
            .collect();

        let eight_minus_four_minus_segment_1_minus_segment_5 = sub_char_from_hashset(
            &sub_char_from_hashset(&eight_minus_four, segment_1),
            segment_5);

        let segment_4 = *eight_minus_four_minus_segment_1_minus_segment_5.iter().collect::<Vec<&char>>().first().unwrap().clone();

        // Deduce 5
        let mut segment_6= 'x';
        for el in two_and_five_and_three {
            if el.contains(segment_0) {
                // we know that this must be the five
                segment_6 = *string_to_char_set(el)
                    .difference(&HashSet::<char>::from_iter([
                        segment_0,
                        segment_1,
                        segment_3,
                        segment_5])).map(|c| c.clone())
                    .collect::<HashSet<char>>()
                    .iter().collect::<Vec<&char>>()
                    .first().unwrap().clone();
            }
        }

        // deduce section 2 (c)
        let segment_2 = *sub_char_from_hashset(&string_to_char_set(&one), segment_6).iter().collect::<Vec<&char>>().first().unwrap().clone();
        self.segments_to_letters[1] = Some(segment_1);
        self.segments_to_letters[5] = Some(segment_5);
        self.segments_to_letters[3] = Some(segment_3);
        self.segments_to_letters[0] = Some(segment_0);
        self.segments_to_letters[4] = Some(segment_4);
        self.segments_to_letters[6] = Some(segment_6);
        self.segments_to_letters[2] = Some(segment_2);

    }

}

/// Represents the input lines
#[derive(Debug)]
struct SegmentLine {
    /// Part before the |-delimiter
    input_part: Vec<String>,
    /// part after
    output_part: Vec<String>
}

impl SegmentLine {
    pub fn from_input_line(line: &String) -> Self {
        let parts: Vec<&str> = line.split(" | ").collect();
        let first_part = parts.get(0).unwrap();
        let second_part = parts.get(1).unwrap();

        return Self {
            input_part: first_part.split(' ').map(|i| String::from(i)).collect(),
            output_part: second_part.split(' ').map(|i| String::from(i)).collect()
        }
    }


    pub fn count_output_digit_1(&self) -> u64 {
        return filter_by_length(&self.output_part, 2).len() as u64;
    }

    pub fn count_output_digit_4(&self) -> u64 {
        return filter_by_length(&self.output_part, 4).len() as u64;
    }

    pub fn count_output_digit_7(&self) -> u64 {
        return filter_by_length(&self.output_part, 3).len() as u64;
    }

    pub fn count_output_digit_8(&self) -> u64 {
        return filter_by_length(&self.output_part, 7).len() as u64;
    }
}

/// Helper function to remove a single char from a HashMap
pub fn sub_char_from_hashset(set: &HashSet<char>, c: char) -> HashSet<char> {
    let other = HashSet::<_>::from_iter([c]);
    set.difference(&other).map(|d| d.clone()).collect()
}

/// Helper function to turn each char into a HashSet Entry
pub fn string_to_char_set(line: &String) -> HashSet<char> {
    line.chars().collect()
}

/// Filters a list of inputs by length
pub fn filter_by_length(input: &Vec<String>, desired_length: u64) -> Vec<&String> {
    input.iter().filter(|line| line.len() == desired_length as usize).collect()
}

/// returns a [SegmentLine] for each input String
fn to_segment_lines(data: &Vec<String>) -> Vec<SegmentLine> {
    return data.iter().map(|line| SegmentLine::from_input_line(line)).collect();
}

pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 8 task 2");
    let segment_lines = to_segment_lines(data);
    let mut sum_of_requested_numbers = 0 as u64;
    for line in segment_lines.iter() {
        sum_of_requested_numbers +=
            line.count_output_digit_1() +
            line.count_output_digit_4() +
            line.count_output_digit_7() +
            line.count_output_digit_8();
    }
    println!("Segments: {:?}", segment_lines);
    return sum_of_requested_numbers;
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 8 task 2");
    let segment_lines = to_segment_lines(data);
    let mut sum: u64 = 0;
    for line in segment_lines {
        let mut clock = SegmentClock::from_segment_line(line);
        clock.deduce_segments();
        sum += clock.output_to_number();
        println!("{:?}", clock);
    }
    return sum;
}