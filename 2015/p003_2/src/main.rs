use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut santa_px = 0;
    let mut santa_py = 0;
    let mut robot_px = 0;
    let mut robot_py = 0;
    let mut santa_turn = true;
    let mut visited_houses = HashSet::new();
    for c in input.chars() {
        if santa_turn {
            santa_px += match c {
                '<' => 1,
                '>' => -1,
                _ => 0
            };
            santa_py += match c {
                '^' => 1,
                'v' => -1,
                _ => 0
            };

            visited_houses.insert((santa_px, santa_py));
        } else {
            robot_px += match c {
                '<' => 1,
                '>' => -1,
                _ => 0
            };
            robot_py += match c {
                '^' => 1,
                'v' => -1,
                _ => 0
            };

            visited_houses.insert((robot_px, robot_py));
        }

        santa_turn = !santa_turn;
    }

    println!("Houses visited: {}", visited_houses.len());
}