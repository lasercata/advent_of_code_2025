use std::fs;

// #[derive(Debug)]
struct IdRange {
    first: u128,
    last: u128
}

impl IdRange {
    /// Sums the invalid IDs
    fn sum_invalid(&self) -> u128 {
        let mut c: u128 = 0;

        for id in self.first .. self.last + 1 {
            if !IdRange::is_id_valid(&id.to_string()) {
                c += id;
            }
        }

        c
    }

    /// Checks if an ID is valid
    fn is_id_valid(id: &str) -> bool {
        let len: usize = id.chars().count();
        let mid_idx: usize = len / 2;

        if len % 2 == 0 && id[..mid_idx] == id[mid_idx..] {
            false
        }
        else {
            true
        }
    }
}

/// Process the file to a list of id ranges
fn process_file(filename: &str) -> Vec<IdRange> {
    let file_content = fs::read_to_string(filename).expect("File error");
    let mut ret: Vec<IdRange> = Vec::new();

    for str_rng in file_content.split(',') {
        let extrema: Vec<&str> = str_rng.trim().split('-').collect();

        let rng = IdRange {
            first: extrema[0].parse().unwrap(),
            last: extrema[1].parse().unwrap(),
        };

        ret.push(rng);
    }

    ret
}

pub fn sol_1(filename: &str) -> u128 {
    let id_ranges = process_file(filename);
    // println!("IdRange: {id_ranges:#?}");
    let mut sum_invalid: u128 = 0;

    for id_range in id_ranges {
        sum_invalid += id_range.sum_invalid();
    }

    sum_invalid
}

