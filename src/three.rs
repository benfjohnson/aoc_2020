type Tree = ();

type Forest = Vec<Vec<Option<Tree>>>;

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

fn input_lines_to_forest(input: &Vec<String>) -> Forest {
    input
        .iter()
        .map(|i| input_line_to_row(i.split("").collect()))
        .collect()
}

// Core logic used for both part 1 and 2
fn get_num_trees(input: &Vec<String>, (x_slope, y_slope): (usize, usize)) -> usize {
    let forest: Forest = input_lines_to_forest(input);

    // How wide is our forest?
    let forest_width: usize = forest[0].len();

    // We can't rely on our current row index to solely determine the x position
    // in a particular iteration, because if we've skipped a row (early return)
    // we shouldn't again walk forward `x_slope` number of characters

    // Said another way, with a slope of (1, 2), after including/counting (0, 0),
    // our next spot should be (1, 2), not (2, 2) (0+1+1 after three iterations)
    let mut current_x_pos = 0;

    let sum_trees = forest.iter().enumerate().fold(0, |sum, (i, row)| {
        // We should conditionally check rows based on whether we've
        // sufficiently moved far enough down the Y axis
        if i % y_slope != 0 {
            return sum;
        }

        // Use modulus to wrap back around to the beginning of the row
        // (this effectively "repeats" our input map horizontally)
        let wrapping_x_position = current_x_pos % forest_width;
        // Increment x_pos now that we've found the next position to check.
        current_x_pos += x_slope;

        match row[wrapping_x_position] {
            Some(_) => sum + 1,
            None => sum,
        }
    });

    sum_trees
}

pub fn get_answers(input: Vec<String>) -> (usize, usize) {
    let answer_one = get_num_trees(&input, (3, 1));
    let answer_two = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|coords| {
            let a = get_num_trees(&input, *coords);
            println!("Coords {:?} return {}", coords, a);
            a
        })
        .fold(1, |product, num| product * num);

    (answer_one, answer_two)
}
