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

    /// Counts the number of fresh elements (the number of elements in the range)
    fn nb_fresh(&self) -> u128 {
        self.last - self.first + 1
    }

    /// Calculates the overlap with `other`.
    /// Prerequisites: self.first < other.first
    fn calc_overlap(&self, other: &Self) -> u128 {
        if self.last < other.first { // No overlap at all
            0
        }
        // else if other.last <= self.last { // Other is included inside Self
        //     other.nb_fresh()
        // }
        else { // Overlap
            self.last - other.first + 1
        }
    }

    /// Checks if Self in included in Other
    fn is_included(&self, other: &Self) -> bool {
        self.first <= other.first &&
            other.last <= self.last
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

    /// Counts the number of fresh elements in all the ranges.
    /// Avoid duplicates.
    ///
    /// This is not optimized, it (tries to) create a vector with 10^14 elements (rough estimation)
    fn nb_fresh_v0(&self) -> u128 {
        let mut fresh: Vec<u128> = Vec::new();

        for r in &self.ranges {
            for id in r.first .. r.last + 1 {
                if ! fresh.contains(&id) {
                    fresh.push(id);
                }
            }
        }

        fresh.len() as u128
    }

    /// Sorts the vector on (first, last) and removes ranges included in others
    fn preprocess(&mut self) {
        // First, sort the vector by `.first` (and in case of equality, on `.last`)
        self.ranges.sort_by_key(|p| (p.first, p.last));

        let mut r_idx: usize = 0;
        while r_idx < self.ranges.len() - 1 {
            if self.ranges[r_idx].is_included(&self.ranges[r_idx + 1]) {
                self.ranges.remove(r_idx + 1);
            }
            else {
                r_idx += 1;
            }
        }
    }

    /// Counts the number of fresh elements in all the ranges.
    /// Avoid duplicates.
    fn nb_fresh(&mut self) -> u128 {
        self.preprocess();

        let mut nb_tot_fresh: u128 = 0;

        for r_idx in 0 .. self.ranges.len() {
            // Last one does not overlap
            if r_idx == self.ranges.len() - 1 {
                nb_tot_fresh += self.ranges[r_idx].nb_fresh();
            }
            else {
                nb_tot_fresh += self.ranges[r_idx].nb_fresh() - self.ranges[r_idx].calc_overlap(&self.ranges[r_idx + 1]);
                // println!("rng: {:02}-{:02},\t overlap: {}", self.ranges[r_idx].first, self.ranges[r_idx].last, self.ranges[r_idx].calc_overlap(&self.ranges[r_idx + 1]));
            }
        }

        nb_tot_fresh
    }
}

pub fn sol_2(filename: &str) -> u128 {
    // let file_content = fs::read_to_string(filename).expect("File error");

    let file = File::open(filename).expect("File error occured");
    let reader = BufReader::new(file);

    let mut ranges: Ranges = Ranges::new();

    for line in reader.lines() {
        let l = line.unwrap();

        if l == "" {
            break;
        }

        ranges.push(IdRange::new(&l));
    }

    ranges.nb_fresh()
}


