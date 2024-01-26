use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut nice_strings_count = 0;
    for line in content.lines() {
        if is_nice_string(line.to_string()) {
            nice_strings_count += 1;
        }
    }

    println!("Nice strings count: {}", nice_strings_count);
}

fn is_nice_string(string: String) -> bool {
    let mut vowel_count = 0;
    let mut has_double_letter = false;
    let mut has_forbidden_string = false;
    let mut characters = string.chars();
    let mut previous_char = characters.next().unwrap();
    if is_vowel(&previous_char) {
        vowel_count += 1;
    }

    for c in characters {

        if is_vowel(&c) {
            vowel_count += 1;
        }

        if c == previous_char {
            has_double_letter = true;
        }

        if (previous_char == 'a' && c == 'b') ||
           (previous_char == 'c' && c == 'd') ||
           (previous_char == 'p' && c == 'q') ||
           (previous_char == 'x' && c == 'y') {
            has_forbidden_string = true;
        }

        previous_char = c;
    }

    vowel_count >= 3 && has_double_letter && !has_forbidden_string
}

fn is_vowel(c: &char) -> bool {
    *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u'
}
