use std::collections::BTreeSet;

pub type ProductNbr = u64;
pub type ProductNbrColl = BTreeSet<ProductNbr>;

pub fn check_number_range(begin: ProductNbr, end_inclusive: ProductNbr) -> ProductNbrColl {
    let mut invalid_numbers   = ProductNbrColl::new();
    for i in begin..=end_inclusive {
        if is_number_invalid(i) {
            invalid_numbers.insert(i);
        }
    }
    invalid_numbers
}

fn is_number_invalid(nbr: ProductNbr) -> bool {
    let nbr_str = nbr.to_string();
    if nbr_str.len() % 2 == 1 {
        return false;
    }
    let split_idx = nbr_str.len() / 2;
    nbr_str[..split_idx] == nbr_str[split_idx..]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        test_range(11, 22, &[11, 22]);
        test_range(95, 115, &[99]);
        test_range(998, 1012, &[1010]);
        test_range(1188511880, 1188511890, &[1188511885]);
        test_range(222220, 222224, &[222222]);
        test_range(1698522, 1698528, &[]);
        test_range(446443, 446449, &[446446]);
        test_range(38593856, 38593862, &[38593859]);
        test_range(565653, 565659, &[]);
        test_range(824824821, 824824827, &[]);
        test_range(2121212118, 2121212124, &[]);
    }

    fn test_range(begin : ProductNbr, end_inclusive: ProductNbr, invalid_nbrs: &[ProductNbr]) {
        assert_eq!(check_number_range(begin, end_inclusive), invalid_nbrs.iter().copied().collect());
    }
}