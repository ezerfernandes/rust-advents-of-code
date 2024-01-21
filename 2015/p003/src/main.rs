use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut px = 0;
    let mut py = 0;
    let mut visited_houses = HashSet::new();
    for c in input.chars() {
        match c {
            '^' => {
                py += 1;
            },
            'v' => {
                py -= 1;

            },
            '<' => {
                px -= 1;
            },
            '>' => {
                px += 1;
            },
            _ => ()
        };

        visited_houses.insert((px, py));
    }

    println!("Houses visited: {}", visited_houses.len());
}