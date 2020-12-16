mod one;
mod two;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file found.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|x| x.expect("Could not parse line"))
        .collect()
}

fn main() {
    let day_one_input: Vec<i32> = lines_from_file("./input-files/one.txt")
        .iter()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    println!(
        "The answer to day one is: {}",
        one::sum_expenses(day_one_input.clone())
    );

    println!(
        "Three nums that add up to 2020 are: {}",
        one::find_three_values(day_one_input)
    );

    let day_two_answer: usize = lines_from_file("./input-files/two.txt")
        .iter()
        .map(two::line_to_valid_info)
        .filter(two::password_is_valid)
        .count();

    let day_two_pt_2_answer: usize = lines_from_file("./input-files/two.txt")
        .iter()
        .map(two::line_to_valid_info)
        .filter(two::password_is_valid_v2)
        .count();

    println!(
        "The answer to the first part of day 2 is: {}. For pt. 2, the answer is {}!",
        day_two_answer, day_two_pt_2_answer
    );
}
