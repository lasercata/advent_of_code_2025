use std::fs::File;
use std::io::{BufReader, BufRead};

struct IdRange {
    first: u128,
    last: u128,
}

impl IdRange {
    /// Creates a new IdRange from a string from the data file
    fn new(line: &str) -> Self {
        let numbers: Vec<&str> = line.split('-').collect();

        IdRange { first: numbers[0].parse().unwrap(), last: numbers[1].parse().unwrap() }
    }

    /// Checks if `id` is in the range
    fn contains(&self, id: u128) -> bool {
        self.first <= id && id <= self.last
    }
}

struct Ranges {
    ranges: Vec<IdRange>,
}

impl Ranges {
    /// Creates a new Ranges
    fn new() -> Self {
        Ranges { ranges: Vec::new() }
    }

    /// Adds `id_range` to the vector of ranges
    fn push(&mut self, id_range: IdRange) {
        self.ranges.push(id_range)
    }

    /// Checks if `id` is in any of the ranges
    fn is_fresh(&self, id: u128) -> bool {
        for r in &self.ranges {
            if r.contains(id) {
                return true
            }
        }

        false
    }
}

pub fn sol(filename: &str) -> u128 {
    // let file_content = fs::read_to_string(filename).expect("File error");

    let file = File::open(filename).expect("File error occured");
    let reader = BufReader::new(file);

    let mut reading_first_part: bool = true; // First part: id ranges; second part: IDs

    let mut nb_fresh: u128 = 0;
    let mut ranges: Ranges = Ranges::new();

    for line in reader.lines() {
        let l = line.unwrap();

        if l == "" {
            reading_first_part = false;
            continue;
        }

        if reading_first_part {
            ranges.push(IdRange::new(&l));
        }
        else {
            let id: u128 = l.parse().unwrap();

            if ranges.is_fresh(id) {
                nb_fresh += 1
            }
        }
    }

    nb_fresh
}

