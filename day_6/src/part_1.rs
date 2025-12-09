use std::fs::File;
use rev_lines::RevLines;

enum OperationType {
    Sum,
    Multiplication,
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

/// Represents the whole worksheet (all the columns)
struct CephalopodWorksheet {
    columns: Vec<CephalopodColumn>,
}

impl CephalopodWorksheet {
    /// Returns a new empty CephalopodWorksheet.
    fn new() -> Self {
        CephalopodWorksheet { columns: Vec::new() }
    }

    /// Initialises a CephalopodWorksheet from the last line of the given file (representing the
    /// operations)
    fn init(&mut self, line: &str) {
        // let mut columns: Vec<CephalopodColumn> = Vec::new();

        for c in line.split(" ") {
            match c {
                "+" => self.columns.push(CephalopodColumn::new(OperationType::Sum)),
                "*" => self.columns.push(CephalopodColumn::new(OperationType::Multiplication)),
                _ => (),
            }
        }
    }

    /// Updates all columns from the given line
    fn update_from_line(&mut self, line: &str) {
        let mut col_idx: usize = 0;

        for c in line.split(" ") {
            if c != "" {
                let n: u128 = c.parse().unwrap();
                self.columns[col_idx].update(n);
                col_idx += 1;
            }
        }
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

pub fn sol_1(filename: &str) -> u128 {
    let file = File::open(filename).unwrap();
    let rev_lines = RevLines::new(file); // Reading lines in reverse order (as + and * are
                                         // commutative, this is to get the operations first)

    let mut worksheet: CephalopodWorksheet = CephalopodWorksheet::new();
    let mut reading_operators: bool = true;

    for line in rev_lines {
        let l: String = line.unwrap();

        if reading_operators {
            worksheet.init(&l);
            reading_operators = false;
        }
        else {
            worksheet.update_from_line(&l);
        }
    }

    worksheet.get_total()
}
