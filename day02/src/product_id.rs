use std::{collections::BTreeSet};

pub type ProductNbr = u64;
pub type ProductNbrColl = BTreeSet<ProductNbr>;

pub fn check_number_range_1(begin: ProductNbr, end_inclusive: ProductNbr) -> ProductNbrColl {
    let mut invalid_numbers   = ProductNbrColl::new();
    for i in begin..=end_inclusive {
        if is_number_invalid_1(i) {
            invalid_numbers.insert(i);
        }
    }
    invalid_numbers
}

fn is_number_invalid_1(nbr: ProductNbr) -> bool {
    let nbr_str = nbr.to_string();
    if nbr_str.len() % 2 == 1 {
        return false;
    }
    let split_idx = nbr_str.len() / 2;
    nbr_str[..split_idx] == nbr_str[split_idx..]
}

pub fn check_number_range_2(begin: ProductNbr, end_inclusive: ProductNbr) -> ProductNbrColl {
    let mut invalid_numbers   = ProductNbrColl::new();
    for i in begin..=end_inclusive {
        if is_number_invalid_2(i) {
            invalid_numbers.insert(i);
        }
    }
    invalid_numbers
}

fn is_number_invalid_2(nbr: ProductNbr) -> bool {
    let nbr_str = nbr.to_string();
    let nbr_str_len = nbr_str.len();
    let steps = get_step_sizes(nbr_str_len);
    for step in steps {
        let mut has_sequence = true;
        for offset_left in (0..nbr_str_len-step).step_by(step) {
            let offset_middle = offset_left + step;
            let offset_right = offset_middle + step;
            if nbr_str[offset_left..offset_middle] != nbr_str[offset_middle..offset_right]{
                has_sequence = false;
                break;
            }
        }
        if has_sequence {
            return true;
        }
    }
    false
}

fn get_step_sizes(str_len: usize) -> Vec<usize>
{
    match str_len {
        1 => vec![],
        2 => vec![1],
        3 => vec![1],
        4 => vec![1, 2],
        5 => vec![1],
        6 => vec![1, 2, 3],
        7 => vec![1],
        8 => vec![1, 2, 4],
        9 => vec![1, 3],
        10 => vec![1, 2, 5],
        _ => panic!("length: {} is not foreseen", str_len),
    }
 }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        test_range(11, 22, &[11, 22], &[11, 22]);
        test_range(95, 115, &[99], &[99, 111]);
        test_range(998, 1012, &[1010], &[999, 1010]);
        test_range(1188511880, 1188511890, &[1188511885], &[1188511885]);
        test_range(222220, 222224, &[222222], &[222222]);
        test_range(1698522, 1698528, &[], &[]);
        test_range(446443, 446449, &[446446], &[446446]);
        test_range(38593856, 38593862, &[38593859], &[38593859]);
        test_range(565653, 565659, &[], &[565656]);
        test_range(824824821, 824824827, &[], &[824824824]);
        test_range(2121212118, 2121212124, &[], &[2121212121]);
    }

    fn test_range(begin : ProductNbr, end_inclusive: ProductNbr, invalid_nbrs_1: &[ProductNbr], invalid_nbrs_2: &[ProductNbr]) {
        assert_eq!(check_number_range_1(begin, end_inclusive), invalid_nbrs_1.iter().copied().collect());
        assert_eq!(check_number_range_2(begin, end_inclusive), invalid_nbrs_2.iter().copied().collect());
    }
}