mod one;

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
}
