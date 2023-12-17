use std::{
    f32::RADIX,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = get_line_buffer("input.txt");
    for line_index in 0..lines.len() {
        let mut line_value: Vec<u32> = vec![];
        let letters: Vec<char> = lines[line_index].chars().collect();
        for letter_index in 0..letters.len() {
            if letters[letter_index].is_numeric() {
                line_value.push(letters[letter_index].to_digit(RADIX).unwrap());
            }
        }
    }
}

fn get_line_buffer(file: impl AsRef<Path>) -> Vec<String> {
    let input = File::open("input.txt").expect("Failed to open file");
    let line_buffer = BufReader::new(input);
    line_buffer
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect()
}
