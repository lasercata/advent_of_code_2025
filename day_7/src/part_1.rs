use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct TachyonManifold {
    len: usize, // length of a row
    nb_splits: u32,
    beam_indexes: HashSet<usize>, // Indexes of the beams in the previous row
}

impl TachyonManifold {
    /// Creates a new empty TachyonManifold.
    fn new() -> Self {
        TachyonManifold { len: 0, nb_splits: 0, beam_indexes: HashSet::new() }
    }


    /// Initiates a new TachyonManifold from the first line of the puzzle input
    fn init(&mut self, first_line: &str) {
        self.len = first_line.len();
        self.nb_splits = 0;
        self.beam_indexes.insert(first_line.find('S').unwrap());
    }

    /// Updates from a line
    fn update_from_line(&mut self, line: &str) {
        let mut split_indexes: Vec<usize> = Vec::new();

        for beam_idx in &self.beam_indexes {
            let corresponding_char: char = line.chars().nth(*beam_idx).unwrap();

            if corresponding_char == '^' {
                self.nb_splits += 1;
                split_indexes.push(*beam_idx); // Store as we cannot edit the HashSet while reading it

            }
        }

        // Update the indexes that were splitted
        for beam_idx in split_indexes {
            self.beam_indexes.remove(&beam_idx);
            self.beam_indexes.insert(beam_idx - 1);
            self.beam_indexes.insert(beam_idx + 1);
        }
    }
}

pub fn sol(filename: &str) -> u32 {
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

    manifold.nb_splits
}
