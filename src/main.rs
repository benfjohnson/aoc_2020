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
    //                TODO: Change me!
    const DAY: &str = "two";

    let puzzle_input = lines_from_file(format!("./input-files/{}.txt", DAY));
    //                         TODO: Change me!
    let (answer_1, answer_2) = two::get_answers(puzzle_input);

    println!(
        "The answers to part 1 and 2 of day {} are {} and {}!",
        DAY, answer_1, answer_2
    );
}
