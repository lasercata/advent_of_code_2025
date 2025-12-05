mod part_1;
mod part_2;

use crate::part_1::sol_1;
use crate::part_2::sol_2;

fn main() {
    println!("Number of 0 (part 1, data test): {}", sol_1("../data/data_test.txt"));
    println!("Number of 0 (part 2, data test): {}", sol_2("../data/data_test.txt"));

    println!("Number of 0 (part 1): {}", sol_1("../data/data_1.txt"));
    println!("Number of 0 (part 2): {}", sol_2("../data/data_1.txt"));
}
