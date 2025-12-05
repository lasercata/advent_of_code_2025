use std::fs;

/// Analyses the line and calculates the next value
fn process_line(current_value: i32, line: &str) -> i32 {
    let num: i32 = line[1..].trim().parse().expect("Non number found!");

    if line.chars().next() == Some('L') {
        (current_value - num) % 100
    }
    else {
        (current_value + num) % 100
    }
}

pub fn sol_1(filename: &str) -> u32 {
    let file_content = fs::read_to_string(filename).expect("File error");

    let mut value: i32 = 50;
    let mut counter: u32 = 0;

    for line in file_content.lines() {
        value = process_line(value, &line);

        if value == 0 {
            counter += 1;
        }
    }

    counter
}


