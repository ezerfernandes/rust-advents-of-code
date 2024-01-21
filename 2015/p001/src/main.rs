use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut floor = 0;
    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }

    println!("Floor: {}", floor);
}
