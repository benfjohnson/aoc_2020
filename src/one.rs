use std::collections::HashSet;
use std::iter::FromIterator;

fn sum_expenses(items: &Vec<i32>) -> i32 {
    // First, read everything into a HashSet
    let expense_set: HashSet<i32> = HashSet::from_iter(items.iter().cloned());
    // Then for each item, determine if the difference between itself and 2020 exists already
    let correct_input = items
        .iter()
        .find(|x| expense_set.contains(&(2020 as i32 - *x)))
        .unwrap();

    let second_input = 2020 - correct_input;

    correct_input * second_input
}

fn find_three_values(items: &Vec<i32>) -> i32 {
    let expense_set: HashSet<i32> = HashSet::from_iter(items.iter().cloned());

    for i in items.iter() {
        for j in items.iter() {
            if expense_set.contains(&(2020 - i - j)) {
                let third_val = expense_set.get(&(2020 - i - j)).unwrap();
                return i * j * third_val;
            }
        }
    }

    -1
}

pub fn get_answers(input: Vec<String>) -> (i32, i32) {
    let nums: Vec<i32> = input.iter().map(|i| i.parse::<i32>().unwrap()).collect();

    (sum_expenses(&nums), find_three_values(&nums))
}
