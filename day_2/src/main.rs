mod part_1;
mod part_2;

// use crate::part_1;
// use crate::part_2;

fn main() {
    println!("On test data:");
    println!("    part 1: {}", part_1::sol("data/example.txt"));
    println!("    part 2: {}", part_2::sol("data/example.txt"));

    println!("\nOn instance data:");
    println!("    part 1: {}", part_1::sol("data/data.txt"));
    println!("    part 2: {}", part_2::sol("data/data.txt"));
}
