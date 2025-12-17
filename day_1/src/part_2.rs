use std::fs;

/// Analyses the line and calculates the next value along with the number of times 0 is crossed.
fn process_line_2(current_value: i32, line: &str) -> (i32, i32) {
    let num: i32 = line[1..].trim().parse().expect("Non number found!");

    let new_value_raw: i32 =
        if line.chars().next() == Some('L') {
            current_value - num
        }
        else {
            current_value + num
        };

    let new_value: i32 = ((new_value_raw % 100) + 100) % 100;
    let c0: i32 = (new_value_raw / 100).abs();
    let c: i32 =
        if (current_value != 0 && new_value_raw < 0) || (new_value == 0 && new_value_raw <= 0) {
            1 + c0
        }
        else {
            c0
        };

    (new_value, c)
}

pub fn sol(filename: &str) -> u32 {
    let file_content = fs::read_to_string(filename).expect("File error");

    let mut value: i32 = 50;
    let mut counter: u32 = 0;

    for line in file_content.lines() {
        let c: i32;
        (value, c) = process_line_2(value, &line);
        // println!("line: {line},\t value: {value},\t c: {c}");
        let c: u32 = c.try_into().unwrap();

        counter += c;
    }

    counter
}


