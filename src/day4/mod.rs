use std::collections::HashSet;
use std::ops::Div;

/// Containins a field for the all the numbers being drawn and
/// the linearized(!) matrices of numbers for the Bingo sheets
#[derive(Debug)]
struct GameWorld<T> {

    /// These are the numbers which will be drawn "randomly" (not here, in fact)
    drawn_numbers: Vec<T>,

    /// Each player has one sheet (or multiple if you happen to have many arms ;-))
    sheets: Vec<Vec<T>>,

    /// Sheet width (as in columns)
    sheet_width: usize,
}

impl<T: std::str::FromStr + Copy> GameWorld<T> {

    /// parses the input and creates a field with all information contained
    pub fn from_raw_input(data: &Vec<String>) -> GameWorld<T> {

        // First line contains the numbers to be drawn
        let first_line = data.get(0).expect("No input given");

        // collects all sheets
        let mut sheets: Vec<Vec<T>> = Vec::new();

        // collects the current sheet we are parsing
        let mut curr_sheet: Vec<T> = Vec::new();

        // stores the with (as in columns) of one sheet
        let mut sheet_width = 0;

        // Skip the first two lines (drawn numbers + empty input) an read from there
        for (i, line) in data.iter().skip(2).enumerate() {
            // Skip all empty guys
            if line.len() == 0 {
                continue;
            }

            // Get all numbers
            let mut line_split: Vec<T> = parse_line_of_numbers::<T>(line, ' ');

            // Remember the sheet width (# numbers we see first)
            if sheet_width == 0 {
                sheet_width = line_split.len();
            }
            // Store this line (as in row) to the current sheet
            curr_sheet.append(&mut line_split);

            // try to get the next line (current index + skipped lines + 1 for next)
            let next_line = data.get(i+2+1);

            // a field is completed when we reach the end of the input
            // or we seen an empty input
            // if it is finished we append the sheet to the list of sheets and
            // empty the current sheet
            if next_line.is_none() || next_line.unwrap().len() == 0{
                sheets.push(curr_sheet.clone());
                curr_sheet.clear();
            }
        }

        // ... just return the gathered information
        return Self {
            drawn_numbers: parse_line_of_numbers::<T>(first_line, ','),
            sheets,
            sheet_width,
        }
    }

}

/// This class represents our bingo game (as in the GameWorld)
/// and all players' progress in markers
#[derive(Debug)]
struct Bingo<'a, T> {

    /// Holds all sheets and the numbers to be drawn
    field: &'a GameWorld<T>,

    /// will store which numbers have been marked (as in the number has been drawn)
    markers: Vec<Vec<bool>>,

    /// points to the number to be drawn next
    pointer: usize,
}

impl<'a, T: std::cmp::PartialEq + std::ops::Add<Output = T> + std::str::FromStr + Copy> Bingo<'a, T> {
    /// Initializer which operates on a [GameWorld]
    pub fn new(field: &'a GameWorld<T>) -> Self {
        return Self {
            field,

            // Creates a boolean Vector for each sheet which will store all the marked entries
            // (false everywhere at the beginning)
            markers: field.sheets
                .iter()
                .map(|f| (0..f.len())
                    .map(|_| false)
                    .collect())
                .collect(),


            pointer: 0,
        }
    }

    /// will simulate a draw of a bingo number
    pub fn draw(&mut self) {

        // Gets the next number which will be drawn from our [GameWorld]
        let curr_number = self.field.drawn_numbers
            .get(self.pointer)
            .expect("There is no number to draw anymore");

        // Now mark all the fields in each sheet which are equal to the dawn number
        for (sheet_num, sheet) in self.field.sheets.iter().enumerate() {
            // .. within each sheet we iterate over each entry
            for (field_entry_num, field_entry) in sheet.iter().enumerate() {
                // ... if the number matches
                if field_entry == curr_number {
                    // we do some rusty in-memory manipulation of the marker boolean
                    let markers = self.markers.get_mut(sheet_num).unwrap();
                    let marker = markers.get_mut(field_entry_num).unwrap();
                    *marker = true;
                }
            }
        }

        // remember which number to draw next
        self.pointer += 1;
    }

    /// Checks if we have some winners and return their
    /// corresponding player index (beginning from 0)
    pub fn compute_winners(&self) -> Vec<usize> {

        // store the winners - remember: there might be multiple winners at a time
        let mut winners: Vec<usize> = Vec::new();

        // Iterate over each sheet
        for (sheet_num, sheet) in self.field.sheets.iter().enumerate() {
            // we basically count for every row
            for row_num in 0..sheet.len().div(self.field.sheet_width) {
                // .. the amount of columns which are marked true
                // abc
                // def
                // -> a+b+c = sum_row_1
                // -> d+e+f = sum_row_2
                let mut sum: u64 = 0;
                for col_num in 0..self.field.sheet_width {
                    sum += self.is_marked(sheet_num, row_num, col_num) as u64;
                }

                // if the sum equals the sheets' width, we have a winner (BINGO condition!)
                if sum == self.field.sheet_width as u64 {
                    winners.push(sheet_num);
                    break; // if the player won already we skip checking if thet o
                }
            }

            // basically the same for vertical direction just invert the directions
            let n_cols = sheet.len().div(self.field.sheet_width);
            for col_num in  0..self.field.sheet_width {
                let mut sum: u64 = 0;
                for row_num in 0..n_cols {
                    sum += self.is_marked(sheet_num, row_num, col_num) as u64;
                }

                if sum == n_cols as u64 {
                    // do check if we already have that one as a winner
                    if !winners.contains(&sheet_num) {
                        winners.push(sheet_num);
                    }
                    break; // if the player won already we skip checking
                }
            }
        }

        return winners;
    }

    /// Checks if a field is marked (i.e., we drawn that number already)
    pub fn is_marked(&self, sheet_num: usize, row_num: usize, col_num: usize) -> bool {
        return *self.markers
            .get(sheet_num)
            .unwrap()
            .get(row_num * self.field.sheet_width + col_num)
            .unwrap()
    }

    /// Checks if there is a number left we could draw
    pub fn can_draw(&self) -> bool {
        return self.pointer < self.field.drawn_numbers.len();
    }

    /// Counts the values of all unmarked fields
    pub fn count_unmarked(&self, sheet_num: usize) -> Result<T, ()> {
        // get the sheet
        let relevant_sheet = self.field.sheets
            .get(sheet_num)
            .expect("Sheet does not exist");

        // and its markers
         let relevant_markers = self.markers
                    .get(sheet_num)
                    .expect("Markers do not exist for that sheet");

        // This is way hacky (I couldn't find a Trait which says "assign a number"
        // but a trait for parse some String to whatever T is... so ya, this
        // is just to set t = 0
        if let Ok(t) = "0".parse::<T>() {
            return Ok((0..relevant_sheet.len())
                .fold(t, |prev, i| {
                    let is_marked = relevant_markers.get(i).unwrap();
                    if !is_marked {
                        let val = relevant_sheet.get(i).unwrap();
                        return *val + prev;
                    }
                    return prev;
                }
            ));
        }
        // the only reason we do this is because we HAVE to return something in the
        // case we could not parse the "0" to a 0. Which can never happen
        // (the compiler however doesn't see this)
        Err(())
    }
}


/// Just parses a String an splits it on [delim].
/// Tries to parse each trimmed split into [T]
/// Ignores empty fields and failed parse attempts
pub fn parse_line_of_numbers<T>(line: &String, delim: char) -> Vec<T> where T: std::str::FromStr {
    let mut data: Vec<T> = Vec::new();
    for split in line.split(delim) {
        if let Ok(maybe_number) = split.trim().parse::<T>() {
            data.push(maybe_number);
        }
    }
    return data;
}

pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 4 task 1");
    let game_field = GameWorld::<u64>::from_raw_input(data);
    let mut game = Bingo::new(&game_field);

    // println!("{:?}", game);

    // while we can draw some number
    while game.can_draw() {
        //... we simulate the draw
        game.draw();

        // Check if we have one winner
        let winners = game.compute_winners();
        if let Some(winner) = winners.get(0) {
            // and if so get the last number drawn and the sum of all unmarked numbers of this sheet
            let last_drawn_number = *game.field.drawn_numbers.get(game.pointer-1).unwrap();
            let unmarked_sum = game.count_unmarked(*winner);
            println!("Winner {}, last drawn number: {}, unmarked sum {:?}",
                     winner,
                     last_drawn_number,
                     unmarked_sum
            );

            // as per definition
            return last_drawn_number * unmarked_sum.unwrap();
        }
    }

    return 0;
}


pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 4 task 2");
    let game_field = GameWorld::<u64>::from_raw_input(data);
    let mut game = Bingo::new(&game_field);

    // Same as above
    let mut last_winners: HashSet<usize> = HashSet::new();
    while game.can_draw() {
        game.draw();

        // We use a set here (which is a mathematical set allowing for set unions etc)
        // to store the winning sheets
        let winners: HashSet<usize> = game.compute_winners().iter().cloned().collect();

        // if the last sheet has one
        if winners.len() == game.field.sheets.len() {
            // find the sheet which is new into the game
            let winner_as_vec = winners
                .difference(&last_winners) // take the difference to the last winners
                .cloned().collect::<Vec<usize>>();
            // and get the (hopefully) only one winner we have new this round
            let winner = *winner_as_vec.get(0).unwrap();

            // do the same as in task one
            let last_drawn_number = *game.field.drawn_numbers.get(game.pointer-1).unwrap();
            let unmarked_sum = game.count_unmarked(winner);
            println!("Last Winner {}, last drawn number: {}, unmarked sum {:?}",
                     winner,
                     last_drawn_number,
                     unmarked_sum
            );

            return last_drawn_number * unmarked_sum.unwrap();
        } else {
            // remember this rounds winners to compare it to the next round winners
            last_winners = last_winners.union(&winners).cloned().collect();
        }
    }
    return 0;

}
