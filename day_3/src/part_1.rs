use std::fs::File;
use std::io::{BufReader, BufRead};

/// Calculates the smallest index of the maximum element.
///
/// E.g for [9, 9, 2, 9], it returns 0.
///
fn arg_max(v: &[u8]) -> usize {
    let mut idx_max = 0;

    for idx in 1 .. v.len() {
        if v[idx] > v[idx_max] {
            idx_max = idx;
        }
    }

    idx_max
}

/// Calculates the largest index of the second maximum element.
///
/// Let's call i_m the index of the maximum element.
/// The 'second maximum' is the maximum of the sub-array at the right of i_m
/// (excluded). If this sub-array is empty (i.e original max is the last
/// element), then we take as sub-array the array prived of i_m.
///
/// E.g:
///     for [9, 9, 2, 9], it returns 3.
///     for [1, 2, 8, 9], it returns 2.
///     for [8, 9, 2, 3], it returns 3.
///
/// In:
///     - v: the vector of positive integers
///     - max_idx: the index of the original maximum
///
fn arg_max_min(v: &Vec<u8>, max_idx: usize) -> usize {
    if max_idx == v.len() - 1 { // Original max is the last element
        arg_max(&v[..max_idx])
    }
    else {
        1 + max_idx + arg_max(&v[1 + max_idx..])
    }
}

/// Finds the max joltage for the given bank (= Vec<u8>)
fn get_max_jolts(v: &Vec<u8>) -> u32 {
    let idx_max = arg_max(v);
    let idx_second_max = arg_max_min(v, idx_max);

    if idx_max < idx_second_max {
        10 * (v[idx_max] as u32) + (v[idx_second_max] as u32)
    }
    else {
        10 * (v[idx_second_max] as u32) + (v[idx_max] as u32)
    }
}

/// Converts a string of numbers to a vector of digits (= bank of batteries' joltage)
fn process_line_to_vec(line: &str) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();

    for c in line.chars() {
        ret.push(c.to_digit(10).expect("Error while parsing char to int") as u8);
    }

    ret
}

pub fn sol(filename: &str) -> u32 {
    // let file_content = fs::read_to_string(filename).expect("File error");

    let file = File::open(filename).expect("File error occured");
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let batt_bank = process_line_to_vec(&line.expect("String error"));

        sum += get_max_jolts(&batt_bank);

        // println!("max jolts: {}", get_max_jolts(&batt_bank));
    }

    sum
}
