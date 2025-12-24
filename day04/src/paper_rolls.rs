use std::cmp::{min, max};
use crate::def::{BoolLine, BoolMatrix};

pub fn is_roll_accessible<const SIZE: usize>(matrix: BoolMatrix<SIZE>, x: usize, y: usize) -> bool
{
    if !matrix[y][x] {
        return false;
    }
    let mut count: usize = 0;
    let from_x = x as i32 - 1;
    let to_x = x as i32 + 1;
    if y > 0 {
        count += count_range(matrix[y - 1], from_x, to_x);
    }
    count += count_range(matrix[y], from_x, to_x);
    if y < SIZE - 1 {
        count += count_range(matrix[y + 1], from_x, to_x)
    }
    count < 5
}

fn count_range<const SIZE: usize>(matrix_line: BoolLine<SIZE>, from_x: i32, to_x: i32) -> usize {
    let start_idx = max(from_x, 0) as usize;
    let end_idx = min(to_x, SIZE as i32 - 1) as usize;
    matrix_line[start_idx..=end_idx].iter().map(|b| if *b {1} else {0}).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const ROLLS: BoolMatrix<10> = [
        //
        [false, false, true , true , false, true , true , true , true , false],
        [true , true , true , false, true , false, true , false, true , true ],
        [true , true , true , true , true , false, true , false, true , true ],
        [true , false, true , true , true , true , false, false, true , false],
        [true , true , false, true , true , true , true , false, true , true ],
        [false, true , true , true , true , true , true , true , false, true ],
        [false, true , false, true , false, true , false, true , true , true ],
        [true , false, true , true , true , false, true , true , true , true ],
        [false, true , true , true , true , true , true , true , true , false],
        [true , false, true , false, true , true , true , false, true , false],
    ];

    #[test]
    fn test() {
        // y=0
        assert_eq!(is_roll_accessible(ROLLS, 2, 0), true);
        assert_eq!(is_roll_accessible(ROLLS, 3, 0), true);
        assert_eq!(is_roll_accessible(ROLLS, 5, 0), true);
        assert_eq!(is_roll_accessible(ROLLS, 6, 0), true);
        assert_eq!(is_roll_accessible(ROLLS, 7, 0), false);
        assert_eq!(is_roll_accessible(ROLLS, 8, 0), true);
        // y=1
        assert_eq!(is_roll_accessible(ROLLS, 0, 1), true);
        assert_eq!(is_roll_accessible(ROLLS, 1, 1), false);
        assert_eq!(is_roll_accessible(ROLLS, 2, 1), false);
        assert_eq!(is_roll_accessible(ROLLS, 4, 1), false);
        assert_eq!(is_roll_accessible(ROLLS, 6, 1), false);
        assert_eq!(is_roll_accessible(ROLLS, 8, 1), false);
        assert_eq!(is_roll_accessible(ROLLS, 9, 1), false);
        // y=2
        assert_eq!(is_roll_accessible(ROLLS, 0, 2), false);
        assert_eq!(is_roll_accessible(ROLLS, 1, 2), false);
        assert_eq!(is_roll_accessible(ROLLS, 2, 2), false);
        assert_eq!(is_roll_accessible(ROLLS, 3, 2), false);
        assert_eq!(is_roll_accessible(ROLLS, 4, 2), false);
        assert_eq!(is_roll_accessible(ROLLS, 6, 2), true);
        assert_eq!(is_roll_accessible(ROLLS, 8, 2), false);
        assert_eq!(is_roll_accessible(ROLLS, 9, 2), false);
        // y=3
        assert_eq!(is_roll_accessible(ROLLS, 0, 3), false);
        assert_eq!(is_roll_accessible(ROLLS, 2, 3), false);
        assert_eq!(is_roll_accessible(ROLLS, 3, 3), false);
        assert_eq!(is_roll_accessible(ROLLS, 4, 3), false);
        assert_eq!(is_roll_accessible(ROLLS, 5, 3), false);
        assert_eq!(is_roll_accessible(ROLLS, 8, 3), false);
        // y=4
        assert_eq!(is_roll_accessible(ROLLS, 0, 4), true);
        assert_eq!(is_roll_accessible(ROLLS, 1, 4), false);
        assert_eq!(is_roll_accessible(ROLLS, 3, 4), false);
        assert_eq!(is_roll_accessible(ROLLS, 4, 4), false);
        assert_eq!(is_roll_accessible(ROLLS, 5, 4), false);
        assert_eq!(is_roll_accessible(ROLLS, 6, 4), false);
        assert_eq!(is_roll_accessible(ROLLS, 8, 4), false);
        assert_eq!(is_roll_accessible(ROLLS, 9, 4), true);
        // y=5
        assert_eq!(is_roll_accessible(ROLLS, 1, 5), false);
        assert_eq!(is_roll_accessible(ROLLS, 2, 5), false);
        assert_eq!(is_roll_accessible(ROLLS, 3, 5), false);
        assert_eq!(is_roll_accessible(ROLLS, 4, 5), false);
        assert_eq!(is_roll_accessible(ROLLS, 5, 5), false);
        assert_eq!(is_roll_accessible(ROLLS, 6, 5), false);
        assert_eq!(is_roll_accessible(ROLLS, 7, 5), false);
        assert_eq!(is_roll_accessible(ROLLS, 9, 5), false);
        // y=6
        assert_eq!(is_roll_accessible(ROLLS, 1, 6), false);
        assert_eq!(is_roll_accessible(ROLLS, 3, 6), false);
        assert_eq!(is_roll_accessible(ROLLS, 5, 6), false);
        assert_eq!(is_roll_accessible(ROLLS, 7, 6), false);
        assert_eq!(is_roll_accessible(ROLLS, 8, 6), false);
        assert_eq!(is_roll_accessible(ROLLS, 9, 6), false);
        // y=7
        assert_eq!(is_roll_accessible(ROLLS, 0, 7), true);
        assert_eq!(is_roll_accessible(ROLLS, 2, 7), false);
        assert_eq!(is_roll_accessible(ROLLS, 3, 7), false);
        assert_eq!(is_roll_accessible(ROLLS, 4, 7), false);
        assert_eq!(is_roll_accessible(ROLLS, 6, 7), false);
        assert_eq!(is_roll_accessible(ROLLS, 7, 7), false);
        assert_eq!(is_roll_accessible(ROLLS, 8, 7), false);
        assert_eq!(is_roll_accessible(ROLLS, 9, 7), false);
        // y=8
        assert_eq!(is_roll_accessible(ROLLS, 1, 8), false);
        assert_eq!(is_roll_accessible(ROLLS, 2, 8), false);
        assert_eq!(is_roll_accessible(ROLLS, 3, 8), false);
        assert_eq!(is_roll_accessible(ROLLS, 4, 8), false);
        assert_eq!(is_roll_accessible(ROLLS, 5, 8), false);
        assert_eq!(is_roll_accessible(ROLLS, 6, 8), false);
        assert_eq!(is_roll_accessible(ROLLS, 7, 8), false);
        assert_eq!(is_roll_accessible(ROLLS, 8, 8), false);
        // y=9
        assert_eq!(is_roll_accessible(ROLLS, 0, 9), true);
        assert_eq!(is_roll_accessible(ROLLS, 2, 9), true);
        assert_eq!(is_roll_accessible(ROLLS, 4, 9), false);
        assert_eq!(is_roll_accessible(ROLLS, 5, 9), false);
        assert_eq!(is_roll_accessible(ROLLS, 6, 9), false);
        assert_eq!(is_roll_accessible(ROLLS, 8, 9), true);
    }

}
