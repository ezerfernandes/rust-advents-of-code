use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut total_area = 0;
    let mut total_ribbon = 0;
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

        // The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face
        let perimeter1 = 2 * (l + w);
        let perimeter2 = 2 * (w + h);
        let perimeter3 = 2 * (h + l);
        let min_perimeter = perimeter1.min(perimeter2).min(perimeter3);
        let volume = l * w * h;
        total_ribbon += min_perimeter + volume;
    }

    println!("Total area: {}", total_area);
    println!("Total ribbon: {}", total_ribbon);
}
