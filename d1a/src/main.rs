use std::fs::File;
use std::vec;
use std::{
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = get_line_buffer("input.txt");
    let mut sums: Vec<u32> = vec![];
    for line_index in 0..lines.len() {
        let mut line_value: Vec<char> = vec![];
        let letters: Vec<char> = lines[line_index].chars().collect();
        for letter_index in 0..letters.len() {
            if letters[letter_index].is_numeric() {
                line_value.push(letters[letter_index]);
            }
            for number in &line_value {
                sums.push(char::to_digit(*number, 10).unwrap());
            }
        }
        let sum: u32 = sums.iter().sum();
        println!("{}", sum);
    }
}

fn get_line_buffer(_file: impl AsRef<Path>) -> Vec<String> {
    let input = File::open("input.txt").expect("Failed to open file");
    let line_buffer = BufReader::new(input);
    line_buffer
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect()
}
