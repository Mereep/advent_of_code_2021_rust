use std::cmp::max;
use std::collections::HashSet;
use crate::day4::parse_line_of_numbers;

/// Represents a instruction to be applied on a Sheet
#[derive(Debug)]
enum Instruction {
    AlongY(usize),
    AlongX(usize),
}

impl Instruction {
    pub fn from_line(line: &String) -> Self {

        // split at the equals sign (rightmost part will be the number)
        let parts = line.split("=").collect::<Vec<_>>();
        let number = (*parts.last().unwrap()).parse::<usize>().unwrap();

        // split the remaining string before `=` on a space.
        // the rightmost part there will be x or y (axis)
        let axis = parts.first().unwrap().split(" ").last().unwrap();

        if axis == "y" {
            return Self::AlongY(number)
        } else if axis == "x" {
            return Self::AlongX(number);
        }

        panic!("Couldn't parse instruction");
    }
}

/// saves a sheet in terms of marked positions [sheet]
/// and a set of instructions
#[derive(Debug)]
struct Sheet {
   sheet: HashSet<(usize, usize)>,
   instructions: Vec<Instruction>,
}

impl Sheet {
    pub fn from_input(lines: &Vec<String>) -> Self {

        // the input is read in a stateful machine
        // we have a state for markers (first part) and a set of instructions
        // (second part; separated by an empty line)
        let mut read_instructions = false;
        let mut sheet = HashSet::<(usize, usize)>::new();
        let mut instructions = Vec::<Instruction>::new();
        for line in lines {

            if !read_instructions {
                // read a marker line
                if line != "" {
                    let numbers = parse_line_of_numbers::<usize>(line, ',');
                    sheet.insert((*numbers.get(0).unwrap(), *numbers.get(1).unwrap()));
                } else {
                    // an empty line triggers a switch to the instruction mode
                    read_instructions = true;
                }
            } else {
                // read instructions
                instructions.push(Instruction::from_line(line));
            }
        }

        Self {
            sheet,
            instructions
        }
    }

    /// finds the highest x value
    fn max_x(&self) -> usize {
        self.sheet.iter().fold(0 as usize, |x, marker| max(x, marker.0))
    }

    /// finds the highest y value
    fn max_y(&self) -> usize {
        self.sheet.iter().fold(0 as usize, |x, marker| max(x, marker.1))
    }

    /// echo the sheet to the terminal
    fn print(&self) {
        for y in 0..=self.max_y() {
           for x in 0..=self.max_x() {
               let p = (x, y);
               if self.sheet.contains(&p) {
                   print!("#")
               } else {
                   print!(" ")
               }
           }
            println!();
        }
    }

    /// apply a instruction at index [index]
    fn apply_instruction(&mut self, index: usize) {
        let instruction = self.instructions.get(index).unwrap();
        match instruction {
            Instruction::AlongY(position) => {
                // find all entries below the y fold
                let after_fold = self.sheet
                    .iter()
                    .filter(|marker| {
                        return marker.1 > *position
                    }).collect::<HashSet<_>>()
                    .iter()
                    .map(|i| **i).collect::<HashSet<_>>();

                // remove all below the fold
                self.sheet.retain(|marker| !after_fold.contains(marker));

                // merge all below fold to the upper part of the sheet
                // this is done by subtracting the distance to the fold from the original
                // y position
                after_fold.iter().for_each(|marker| {
                   self.sheet.insert((marker.0, *position - (marker.1 - *position)));
                });
            }
            Instruction::AlongX(position) => {
                // works the same as above except the roles of x and y are changed
                let after_fold = self.sheet
                    .iter()
                    .filter(|marker| {
                        return marker.0 > *position
                    }).collect::<HashSet<_>>()
                    .iter()
                    .map(|i| **i).collect::<HashSet<_>>();

                self.sheet.retain(|marker| !after_fold.contains(marker));
                after_fold.iter().for_each(|marker| {
                    self.sheet.insert((*position - (marker.0 - *position), marker.1 ));
                });
            }
        }
    }
}

pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 13 task 1");
    let mut sheet = Sheet::from_input(data);
    println!("Sheet before {}", sheet.sheet.len());
    sheet.apply_instruction(0);
    // sheet.apply_instruction(1);
    let markers = sheet.sheet.len() as u64;
    // println!("Sheet {:?}", sheet);
    return markers as u64;
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 13 task 2");
    let mut sheet = Sheet::from_input(data);
    println!("Sheet before {}", sheet.sheet.len());
    for i in 0..sheet.instructions.len() {
        sheet.apply_instruction(i);
    }

    sheet.print();
    let markers = sheet.sheet.len() as u64;
    return markers as u64;
}
