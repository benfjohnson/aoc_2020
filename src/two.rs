struct ValidationInfo {
    password: String,
    required_char: char,
    chars_allowed: (usize, usize),
}

fn get_num_range(s: &str) -> (usize, usize) {
    // expects strings in the format of "X-Y"
    let bounds: Vec<&str> = s.split("-").collect();
    (
        bounds[0].parse::<usize>().unwrap(),
        bounds[1].parse::<usize>().unwrap(),
    )
}

fn line_to_valid_info(l: &String) -> ValidationInfo {
    let sections: Vec<&str> = l.split(" ").collect();

    ValidationInfo {
        password: String::from(sections[2]),
        required_char: sections[1].chars().nth(0).unwrap(),
        chars_allowed: get_num_range(sections[0]),
    }
}

fn password_is_valid(v: &ValidationInfo) -> bool {
    let ValidationInfo {
        password,
        required_char,
        chars_allowed,
    } = v;

    let num_matches = password.matches(*required_char).count();
    num_matches >= chars_allowed.0 && num_matches <= chars_allowed.1
}

// In this rendition, pw is valid if exactly one of the required_chars exist
// at the indicated index
fn password_is_valid_v2(v: &ValidationInfo) -> bool {
    let ValidationInfo {
        password,
        required_char,
        chars_allowed,
    } = v;

    let mut total_occurrences: usize = 0;
    let (lb, ub) = chars_allowed;
    if password.chars().nth(lb - 1).unwrap() == *required_char {
        total_occurrences += 1;
    }

    if password.chars().nth(ub - 1).unwrap() == *required_char {
        total_occurrences += 1;
    }

    total_occurrences == 1
}

pub fn get_answers(input: Vec<String>) -> (usize, usize) {
    let valid_info = input.iter().map(line_to_valid_info);

    (
        valid_info.clone().filter(password_is_valid).count(),
        valid_info.filter(password_is_valid_v2).count(),
    )
}
