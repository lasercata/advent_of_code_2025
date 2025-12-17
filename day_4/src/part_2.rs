use std::fs::File;
use std::io::{BufReader, BufRead};

struct PaperRollMap {
    m: Vec<Vec<char>>,
    nb_lines: usize,
    nb_rows: usize,
}

impl PaperRollMap {
    fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("File error occured");
        let reader = BufReader::new(file);

        let mut map: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let mut map_line: Vec<char> = Vec::new();

            for c in line.expect("String error").chars() {
                map_line.push(c);
            }

            map.push(map_line);
        }

        let nb_lines: usize = map.len();
        let nb_rows: usize = map[0].len();

        PaperRollMap {
            m: map,
            nb_lines: nb_lines,
            nb_rows: nb_rows,
        }
    }

    /// Checks if the coordinates (i, j) points to an existing point in the map
    ///
    /// In:
    ///     - i: the line index
    ///     - j: the row index
    ///
    fn is_pt_valid(&self, i: isize, j: isize) -> bool {
        0 <= i && i < self.m.len() as isize &&
            0 <= j && j < self.m[i as usize].len() as isize
    }

    /// Checks if the point at given coordinates is a paper roll ('@')
    ///
    /// In:
    ///     - i: the line index
    ///     - j: the row index
    ///
    fn is_paper_roll(&self, i: isize, j: isize) -> bool {
        self.is_pt_valid(i, j) && self.m[i as usize][j as usize] == '@'
    }

    /// Counts the number of papers rolls in the eight adjacent positions of the point at (i, j).
    /// Note that it does not checks if there is a @ at this position.
    ///
    /// In:
    ///     - i: the line index
    ///     - j: the row index
    ///
    fn nb_neighbors(&self, i: usize, j: usize) -> u8 {
        let mut n = 0;

        let i_i: isize = i as isize;
        let j_i: isize = j as isize;

        for line in i_i - 1 .. i_i + 2 {
            for row in j_i - 1 .. j_i + 2 {
                if self.is_paper_roll(line, row) {
                    n += 1;
                }
            }
        }

        n
    }

    /// Checks if paper roll at (i, j) is accessible
    /// I.e has fewer than 4 adjacent rolls of papers.
    ///
    /// In:
    ///     - i: the line index
    ///     - j: the row index
    ///
    fn is_accessible(&self, i: usize, j: usize) -> bool {
        self.nb_neighbors(i, j) <= 4
    }

    /// Calculates the number of accessible paper rolls
    /// When encountering one, it removes it.
    fn calc_nb_accessible_and_remove(&mut self) -> u32 {
        let mut nb_accessible: u32 = 0;

        for i in 0 .. self.nb_lines {
            for j in 0 .. self.nb_rows {
                if self.is_paper_roll(i as isize, j as isize) && self.is_accessible(i, j) {
                    self.remove_roll(i, j);
                    nb_accessible += 1;
                }
            }
        }

        nb_accessible
    }

    /// Removes the roll at (i, j)
    ///
    /// In:
    ///     - i: the line index
    ///     - j: the row index
    ///
    fn remove_roll(&mut self, i: usize, j: usize) {
        self.m[i][j] = '.'
    }
}

pub fn sol(filename: &str) -> u32 {
    let mut paper_roll_map = PaperRollMap::new(filename);

    let mut nb_total_accessible: u32 = 0;

    loop {
        let nb_accessible: u32 = paper_roll_map.calc_nb_accessible_and_remove();

        if nb_accessible == 0 {
            break;
        }

        nb_total_accessible += nb_accessible;
    }

    nb_total_accessible
}

