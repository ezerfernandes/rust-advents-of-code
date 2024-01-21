use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut total_area = 0;
    for line in contents.lines() {
        // split using x and get first 3 values.
        let mut dimensions = line.split('x').take(3).map(|x| x.parse::<u32>().unwrap());
        let l = dimensions.next().unwrap();
        let w = dimensions.next().unwrap();
        let h = dimensions.next().unwrap();


        let area1 = l * w;
        let area2 = w * h;
        let area3 = h * l;
        let min_area = area1.min(area2).min(area3);

        total_area += 2 * area1 + 2 * area2 + 2 * area3 + min_area;
    }

    println!("Total area: {}", total_area);
}
