use std::fs;

fn main() {
    let mut matrix: [[bool; 1001]; 1001] = [[false; 1001]; 1001];

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    for line in contents.lines() {


    println!("Hello, world!");
}

struct Point {
    x: usize,
    y: usize,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn turn_on(area: &Rectangle, matrix: &mut [[bool; 1001]; 1001]) {
    for x in area.p1.x..area.p2.x {
        for y in area.p1.x..area.p2.y {
            matrix[x][y] = true;
        }
    }
}
