use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct TachyonManifold {
    len: usize, // length of a row
    nb_splits: u64,
    beam_indexes: HashMap<usize, u64>, // Indexes of the beams in the previous row, along with the
                                       // number of possible ways to get there
}

impl TachyonManifold {
    /// Creates a new empty TachyonManifold.
    fn new() -> Self {
        TachyonManifold { len: 0, nb_splits: 0, beam_indexes: HashMap::new() }
    }


    /// Initiates a new TachyonManifold from the first line of the puzzle input
    fn init(&mut self, first_line: &str) {
        self.len = first_line.len();
        self.nb_splits = 0;
        self.beam_indexes.insert(first_line.find('S').unwrap(), 1);
    }

    /// Updates from a line
    fn update_from_line(&mut self, line: &str) {
        let mut split_indexes: Vec<usize> = Vec::new();
        let mut beam_indexes_new_line: HashMap<usize, u64> = HashMap::new();

        for beam_idx in &self.beam_indexes {
            let corresponding_char: char = line.chars().nth(*beam_idx.0).unwrap();

            if corresponding_char == '^' {
                self.nb_splits += 1;
                split_indexes.push(*beam_idx.0); // Store as we cannot edit the HashSet while reading it

            }
            else {
                beam_indexes_new_line.insert(*beam_idx.0, *beam_idx.1);
            }
        }

        // Update the indexes that were splitted
        for beam_idx in split_indexes {
            let nb_ways_to_get_here: u64 = self.beam_indexes.remove(&beam_idx).unwrap();

            if beam_indexes_new_line.contains_key(&(beam_idx - 1)) {
                let other_nb_ways: u64 = beam_indexes_new_line[&(beam_idx - 1)];
                beam_indexes_new_line.insert(beam_idx - 1, nb_ways_to_get_here + other_nb_ways);
            }
            else {
                beam_indexes_new_line.insert(beam_idx - 1, nb_ways_to_get_here);
            }

            if beam_indexes_new_line.contains_key(&(beam_idx + 1)) {
                let other_nb_ways: u64 = beam_indexes_new_line[&(beam_idx + 1)];
                beam_indexes_new_line.insert(beam_idx + 1, nb_ways_to_get_here + other_nb_ways);
            }
            else {
                beam_indexes_new_line.insert(beam_idx + 1, nb_ways_to_get_here);
            }
        }

        self.beam_indexes = beam_indexes_new_line;
    }

    /// Calculates the total number of possible ways
    fn get_total_ways(&self) -> u64 {
        let mut sum: u64 = 0;

        for beam in &self.beam_indexes {
            sum += beam.1;
        }

        sum
    }
}

pub fn sol(filename: &str) -> u64 {
    let file = File::open(filename).expect("File error occured");
    let reader = BufReader::new(file);

    let mut reading_first_line: bool = true;
    let mut manifold: TachyonManifold = TachyonManifold::new();

    for line in reader.lines() {
        let l = line.unwrap();

        if reading_first_line {
            reading_first_line = false;

            manifold.init(&l);
        }
        else {
            manifold.update_from_line(&l);
        }
    }

    manifold.get_total_ways()
}

