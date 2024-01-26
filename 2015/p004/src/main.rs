use md5;

fn main() {
    let secret_key = "yzbqklnj";
    let mut n = 1;
    let mut digest = String::new();
    while !digest.starts_with("000000") {
        n += 1;
        let input = format!("{}{}", secret_key, n);
        digest = format!("{:x}", md5::compute(input.as_bytes()));
    }

    println!("The lowest positive number is {}.", n);
}
