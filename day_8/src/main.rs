mod structures;
mod part_1;
mod part_2;

// use crate::part_1::sol_1;
// use crate::part_2::sol_2;

fn main() {
    println!("Part 1:");
    println!("    test data:     {}", part_1::sol("data/example.txt", 10));
    println!("    instance data: {}", part_1::sol("data/data.txt", 1000));

    println!("\nPart 2:");
    println!("    test data:     {}", part_2::sol("data/example.txt"));
    println!("    instance data: {}", part_2::sol("data/data.txt"));
}
