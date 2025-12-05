mod part_1;
use crate::part_1::sol_1;

fn main() {
    println!("Result for part 1, test data: {}", sol_1("../data/data_test.txt"));

    println!("Result for part 1, instance data: {}", sol_1("../data/data.txt"));
}
