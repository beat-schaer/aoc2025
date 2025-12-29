mod converter;
mod def;
mod paper_rolls;

use converter::convert_matrix_iter;
use paper_rolls::get_accessible_matrix;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::paper_rolls::remove_all;

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);
    const MATRIX_SIZE: usize = 136;
    let matrix = convert_matrix_iter::<MATRIX_SIZE>(reader.lines().map(|r| r.unwrap()));
    let (_, result_1) = get_accessible_matrix(&matrix);
    let result_2 = remove_all(&matrix);
    println!("*** Result 1 is {} free rolls, result 2 is {} removed rolls ***", result_1, result_2);
}
