#![allow(dead_code)]

mod day1;
mod day2;
use std::fs;
use std::io::{Read};

fn main() {

    /*
        let data = load_data("day1", true);
        let data_as_numbers: Vec<u64> = data.iter().map(|line| line.parse::<u64>().unwrap()).collect();
        let res1 = day1::task1(&data_as_numbers);
        let res2 = day1::task2_functional(&data_as_numbers, 3);
        println!("Result 1: {:?}\nResult 2: {:?}", res1, res2);
    */

    let data = load_data("day2", true);
    let res1 = day2::task1(&data);
    let res2 = day2::task2(&data);
    println!("Result 1: {:?}\nResult 2: {:?}", res1, res2);


}

/// Will load a text file into lines which must be under `/src/dayXY/input.txt`
/// or `/src/dayXY/input.txt` depending on `load_test`
fn load_data(day: &'static str, load_test: bool) -> Vec<String> {
    let file_name = if load_test == false {"input"} else {"testinput"};
    let file_name = format!("./src/{}/{}.txt", day, file_name);
    let mut file = fs::File::open(&file_name)
        .expect(&format!("Couldn't open input file {}", file_name));

    let mut lines = String::new();
    file.read_to_string(&mut lines).expect("Couldn't read data");

    lines.lines().map(|line| {
        String::from(line.trim())
    }).collect()

}