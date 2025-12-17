use std::fs;

/// Transposes the input
///
/// Note that the implementation is not optimized at all: it makes multiple copies of the whole
/// file in vector of vectors (or strings) for convenience.
fn transpose(s: &Vec<&str>) -> Vec<String> {
    let nb_rows: usize = s.len();
    let nb_cols: usize = s[0].len();

    // Create a matrix with the right shape, filled with '.'
    let mut tmp: Vec<Vec<char>> = Vec::new();
    for k in 0 .. nb_cols {
        tmp.push(Vec::new());

        for _ in 0 .. nb_rows {
            tmp[k].push('.');
        }
    }

    // Fill the matrix with the correct values
    for i in 0 .. nb_cols {
        for j in 0 .. nb_rows {
            tmp[i][j] = s[j].chars().nth(i).unwrap();
        }
    }

    // Convert the tmp matrix to a vector of strings
    let mut transposed: Vec<String> = Vec::new();
    for i in 0 .. nb_cols {
        transposed.push(String::new());

        for j in 0 .. nb_rows {
            transposed[i].push(tmp[i][j]);
        }
    }

    transposed
}

enum OperationType {
    Sum,
    Multiplication,
}

impl OperationType {
    fn new(c: char) -> Self {
        match c {
            '+' => OperationType::Sum,
            '*' => OperationType::Multiplication,
            _ => panic!("Invalid char")
        }
    }
}

// Represents a column of "cephalopod" calculus
struct CephalopodColumn {
    operation: OperationType,
    v: u128, // The current value
}

impl CephalopodColumn {
    /// Creates a new CephalopodColumn with the given operation type.
    fn new(op: OperationType) -> Self {
        match op {
            OperationType::Sum => CephalopodColumn { operation: op, v: 0 },
            OperationType::Multiplication => CephalopodColumn { operation: op, v: 1 },
        }
    }

    /// Update the value of self according to the operation type and reading from parameter `v`.
    fn update(&mut self, v: u128) {
        match self.operation {
            OperationType::Sum => self.v += v,
            OperationType::Multiplication => self.v *= v,
        }
    }
}

struct CephalopodWorksheet {
    columns: Vec<CephalopodColumn>,
}

impl CephalopodWorksheet {
    fn new(transposed: &Vec<String>) -> Self {
        let mut init_line: bool = true; // Keep track of type of line read

        let mut calc_cols: Vec<CephalopodColumn> = Vec::new();
        let mut current_idx: usize = 0;

        for line in transposed {
            let l = line.trim();

            if init_line {
                init_line = false;

                // Split the operator and the value
                let op = OperationType::new(l.chars().last().unwrap());

                let n_to_parse: &str = &l[0 .. l.chars().count() - 1].trim();
                let val: u128 = n_to_parse.parse().unwrap();

                calc_cols.push(CephalopodColumn::new(op));
                current_idx = &calc_cols.len() - 1;
                calc_cols[current_idx].update(val);
            }
            else if l == "" {
                init_line = true;
            }
            else {
                let val: u128 = l.parse().unwrap();
                calc_cols[current_idx].update(val);
            }
        }

        CephalopodWorksheet { columns: calc_cols }
    }

    /// Calculates the total (sum of all column's values)
    fn get_total(&self) -> u128 {
        let mut sum: u128 = 0;

        for col in &self.columns {
            sum += col.v;
        }

        sum
    }
}

pub fn sol(filename: &str) -> u128 {
    // First transpose the file
    let file_content = fs::read_to_string(filename).unwrap();

    let mut lines: Vec<&str> = Vec::new();
    for line in file_content.split('\n') {
        if line != "" {
            lines.push(line);
        }
    }

    let transposed = transpose(&lines);

    // for s in &transposed {
    //     println!("{s:?}");
    //     // println!("{:?}", s.trim());
    // }

    // Then parse the transposed file
    let worksheet: CephalopodWorksheet = CephalopodWorksheet::new(&transposed);

    worksheet.get_total()
}
