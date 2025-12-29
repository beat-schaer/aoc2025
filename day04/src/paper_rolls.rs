use crate::def::{BoolLine, BoolMatrix, init_matrix};
use std::cmp::{max, min};

pub fn remove_all<const SIZE: usize>(matrix: &BoolMatrix<SIZE>) -> usize {
    let mut removed = 0;
    let mut work_matrix = *matrix;
    loop {
        let removed_in_round = remove_round(&mut work_matrix);
        if removed_in_round == 0 {
            break;
        }
        removed += removed_in_round;
    }
    removed
}

fn remove_round<const SIZE: usize>(matrix: &mut BoolMatrix<SIZE>) -> usize {
    let (accessible_matrix, _) = get_accessible_matrix(matrix);
    let mut removed = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if accessible_matrix[y][x] {
                matrix[y][x] = false;
                removed += 1;
            }
        }
    }
    removed
}

pub fn get_accessible_matrix<const SIZE: usize>(
    matrix: &BoolMatrix<SIZE>,
) -> (BoolMatrix<SIZE>, usize) {
    let mut result = init_matrix();
    let mut count = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if is_roll_accessible(matrix, x, y) {
                result[y][x] = true;
                count += 1;
            }
        }
    }
    (result, count)
}

fn is_roll_accessible<const SIZE: usize>(matrix: &BoolMatrix<SIZE>, x: usize, y: usize) -> bool {
    if !matrix[y][x] {
        return false;
    }
    let mut count: usize = 0;
    let from_x = x as i32 - 1;
    let to_x = x as i32 + 1;
    if y > 0 {
        count += count_range(&matrix[y - 1], from_x, to_x);
    }
    count += count_range(&matrix[y], from_x, to_x);
    if y < SIZE - 1 {
        count += count_range(&matrix[y + 1], from_x, to_x)
    }
    count < 5
}

fn count_range<const SIZE: usize>(matrix_line: &BoolLine<SIZE>, from_x: i32, to_x: i32) -> usize {
    let start_idx = max(from_x, 0) as usize;
    let end_idx = min(to_x, SIZE as i32 - 1) as usize;
    matrix_line[start_idx..=end_idx]
        .iter()
        .map(|b| if *b { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::converter::convert_matrix;
    use crate::def::StrMatrix;

    const ROLLS: StrMatrix<10> = [
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    const AVAILABLE_ROLLS_1: StrMatrix<10> = [
        "..@@.@@.@.",
        "@.........",
        "......@...",
        "..........",
        "@........@",
        "..........",
        "..........",
        "@.........",
        "..........",
        "@.@.....@.",
    ];

    const ROLLS_ROUND_1: StrMatrix<10> = [
        ".......@..",
        ".@@.@.@.@@",
        "@@@@@...@@",
        "@.@@@@..@.",
        ".@.@@@@.@.",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "..@@@.@@@@",
        ".@@@@@@@@.",
        "....@@@...",
    ];

    const ROLLS_ROUND_2: StrMatrix<10> = [
        "..........",
        ".@@.....@.",
        ".@@@@...@@",
        "..@@@@....",
        ".@.@@@@...",
        "..@@@@@@..",
        "...@.@.@@@",
        "..@@@.@@@@",
        "..@@@@@@@.",
        "....@@@...",
    ];

    const ROLLS_ROUND_3: StrMatrix<10> = [
        "..........",
        "..@.......",
        ".@@@@.....",
        "..@@@@....",
        "...@@@@...",
        "..@@@@@@..",
        "...@.@.@@.",
        "..@@@.@@@@",
        "...@@@@@@.",
        "....@@@...",
    ];

    const ROLLS_ROUND_4: StrMatrix<10> = [
        "..........",
        "..........",
        "..@@@.....",
        "..@@@@....",
        "...@@@@...",
        "...@@@@@..",
        "...@.@.@@.",
        "...@@.@@@.",
        "...@@@@@@.",
        "....@@@...",
    ];

    const ROLLS_ROUND_5: StrMatrix<10> = [
        "..........",
        "..........",
        "...@@.....",
        "..@@@@....",
        "...@@@@...",
        "...@@@@@..",
        "...@.@.@@.",
        "...@@.@@@.",
        "...@@@@@..",
        "....@@@...",
    ];

    const ROLLS_ROUND_6: StrMatrix<10> = [
        "..........",
        "..........",
        "...@@.....",
        "...@@@....",
        "...@@@@...",
        "...@@@@@..",
        "...@.@.@@.",
        "...@@.@@@.",
        "...@@@@@..",
        "....@@@...",
    ];

    const ROLLS_ROUND_7: StrMatrix<10> = [
        "..........",
        "..........",
        "....@.....",
        "...@@@....",
        "...@@@@...",
        "...@@@@@..",
        "...@.@.@@.",
        "...@@.@@@.",
        "...@@@@@..",
        "....@@@...",
    ];

    const ROLLS_ROUND_8: StrMatrix<10> = [
        "..........",
        "..........",
        "..........",
        "...@@@....",
        "...@@@@...",
        "...@@@@@..",
        "...@.@.@@.",
        "...@@.@@@.",
        "...@@@@@..",
        "....@@@...",
    ];

    const ROLLS_ROUND_9: StrMatrix<10> = [
        "..........",
        "..........",
        "..........",
        "....@@....",
        "...@@@@...",
        "...@@@@@..",
        "...@.@.@@.",
        "...@@.@@@.",
        "...@@@@@..",
        "....@@@...",
    ];

    #[test]
    fn test_1() {
        assert_eq!(
            get_accessible_matrix(&convert_matrix(&ROLLS)).0,
            convert_matrix(&AVAILABLE_ROLLS_1)
        );
    }

    #[test]
    fn test_2() {
        let mut work_matrix = convert_matrix(&ROLLS);
        assert_eq!(remove_round(&mut work_matrix), 13);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_1));
        assert_eq!(remove_round(&mut work_matrix), 12);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_2));
        assert_eq!(remove_round(&mut work_matrix), 7);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_3));
        assert_eq!(remove_round(&mut work_matrix), 5);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_4));
        assert_eq!(remove_round(&mut work_matrix), 2);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_5));
        assert_eq!(remove_round(&mut work_matrix), 1);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_6));
        assert_eq!(remove_round(&mut work_matrix), 1);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_7));
        assert_eq!(remove_round(&mut work_matrix), 1);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_8));
        assert_eq!(remove_round(&mut work_matrix), 1);
        assert_eq!(work_matrix, convert_matrix(&ROLLS_ROUND_9));
        assert_eq!(remove_round(&mut work_matrix), 0);
    }
}
