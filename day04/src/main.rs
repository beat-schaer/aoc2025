mod converter;
mod def;
mod paper_rolls;

use std::{fs::File, io::{BufRead, BufReader}};
use def::init_matrix;
use converter::convert_line;
use paper_rolls::is_roll_accessible;

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);
    const MATRIX_SIZE: usize = 136;
    let mut matrix= init_matrix::<MATRIX_SIZE>();
    for (idx, line) in reader.lines().enumerate() {
        matrix[idx] = convert_line(line.as_ref().unwrap());
    }
    let mut result = 0;
    for y in 0..MATRIX_SIZE {
        for x in 0..MATRIX_SIZE {
            if is_roll_accessible(matrix, x, y) {
                result += 1;
            }
        }
    }
    println!("*** Result is {} free rolls ***", result);
}
