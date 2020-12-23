type Tree = ();

type Forest = Vec<Vec<Option<Tree>>>;

const RIGHT_SLOPE: usize = 3;

fn input_line_to_row(input: Vec<&str>) -> Vec<Option<Tree>> {
    input
        .iter()
        .filter(|i| *i == &"." || *i == &"#")
        .map(|i| match i.as_ref() {
            "#" => Some(()),
            _ => None,
        })
        .collect()
}

fn input_lines_to_forest(input: Vec<String>) -> Forest {
    input
        .iter()
        .map(|i| input_line_to_row(i.split("").collect()))
        .collect()
}

// Now we have our Forest type created. We should be able to, for each row of forest,
// iterate over three, and down 1. At some point we'll pass our given input length..
// but if we use the modulo operator we should be able to iterate from the remainder.

fn get_num_trees(input: Vec<String>) -> usize {
    let forest: Forest = input_lines_to_forest(input);

    // As mentioned in the above comment, used for tracking how beyond our input size we've gotten
    let forest_width: usize = forest[0].len();
    println!("Length of forest: {}", forest_width);
    let mut current_latitude: usize = 0;

    // This is the answer to first problem of Day 3.
    let sum_trees = forest.iter().fold(0, |sum, r| {
        let relative_pos = current_latitude % forest_width;
        current_latitude += RIGHT_SLOPE;

        match r[relative_pos] {
            Some(_) => sum + 1,
            None => sum,
        }
    });

    sum_trees
}

pub fn get_answers(input: Vec<String>) -> (usize, usize) {
    (get_num_trees(input), 0)
}
