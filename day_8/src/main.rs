mod part_1;
mod part_2;

use crate::part_1::sol_1;
use crate::part_2::sol_2;

fn main() {
    println!("On test data:");
    println!("    part 1: {}", sol_1("data/example.txt", 10));
    println!("    part 2: {}", sol_2("data/example.txt"));

    println!("\nOn instance data:");
    println!("    part 1: {}", sol_1("data/data.txt", 1000));
    println!("    part 2: {}", sol_2("data/data.txt"));
}
