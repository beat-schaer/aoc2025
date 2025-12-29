use std::iter::Iterator;

use crate::def::{BoolLine, BoolMatrix, StrMatrix, init_line, init_matrix};

pub fn convert_matrix<const SIZE: usize>(input: &StrMatrix<SIZE>) -> BoolMatrix<SIZE> {
    convert_matrix_iter(input.iter().map(|item| item.to_string()))
}

pub fn convert_matrix_iter<const SIZE: usize>(
    iter: impl Iterator<Item = String>,
) -> BoolMatrix<SIZE> {
    let mut matrix = init_matrix();
    for (idx, input) in iter.enumerate() {
        matrix[idx] = convert_line(&input);
    }
    matrix
}

pub fn convert_line<const SIZE: usize>(input: &str) -> BoolLine<SIZE> {
    let mut line = init_line::<SIZE>();
    for n in 0..SIZE {
        line[n] = match input.chars().nth(n) {
            Some('.') => false,
            Some('@') => true,
            _ => panic!("Unexpected / missing char detected"),
        }
    }
    line
}
