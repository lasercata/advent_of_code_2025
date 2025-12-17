use std::fs;

struct ID {
    n: u128
}

impl ID {
    fn new(n: u128) -> Self {
        ID {n: n}
    }

    fn to_string(&self) -> String {
        self.n.to_string()
    }

    /// Check if self.n represents a p-repetition
    /// E.g 121212 is a 2-repetition (of 12)
    ///
    /// In:
    ///     - p: should be in [1 ; len/2].
    /// Out:
    ///     true iff self.n is a p-repetition
    ///
    fn repeat_n(&self, p: usize) -> bool {
        let n_str: String = self.to_string();

        if n_str.len() % p != 0 {
            false
        }
        else {
            let block_nb: usize = n_str.len() / p;
            let block_0: &str = &n_str[.. p];

            for block_id in 1 .. block_nb {
                let block: &str = &n_str[block_id * p .. (block_id + 1) * p];

                if block != block_0 {
                    return false
                }
            }

            true
        }
    }

    /// Checks if an ID is valid
    fn is_valid(&self) -> bool {
        let id: String = self.to_string();

        let len: usize = id.chars().count();
        let mid_idx: usize = len / 2;

        for p in 1 .. mid_idx + 1 {
            if self.repeat_n(p) {
                return false
            }
        }

        true
    }
}

// #[derive(Debug)]
struct IdRange {
    first: u128,
    last: u128
}

impl IdRange {
    /// Sums the invalid IDs
    fn sum_invalid(&self) -> u128 {
        let mut c: u128 = 0;

        for id_n in self.first .. self.last + 1 {
            let id: ID = ID::new(id_n);

            if !id.is_valid() {
                c += id.n;
            }
        }

        c
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

pub fn sol(filename: &str) -> u128 {
    let id_ranges = process_file(filename);
    // println!("IdRange: {id_ranges:#?}");
    let mut sum_invalid: u128 = 0;

    for id_range in id_ranges {
        sum_invalid += id_range.sum_invalid();
    }

    sum_invalid
}

