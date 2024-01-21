use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut floor = 0;
    let mut basement_pos: usize = 0;
    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 && basement_pos == 0 {
            basement_pos = i + 1;
        }
    }

    println!("Floor: {}", floor);
    println!("Basement position: {}", basement_pos);
}
