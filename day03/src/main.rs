mod joltage;

use std::ops::AddAssign;
use std::iter::Sum;
use std::fs::File;
use std::io::{BufRead, BufReader};
use joltage::calc_joltage_1;

use crate::joltage::calc_joltage_2;

struct IntPair(u64, u64);

impl AddAssign for IntPair {
    fn add_assign(&mut self, rhs: Self)
    {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sum for IntPair {
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item = IntPair>
    {
        let mut acc = Self(0,0);
        for i in iter {
            acc += i;
        }
        acc
    }
}

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let totals: IntPair = reader.lines().map(|line| 
        IntPair(calc_joltage_1(line.as_ref().unwrap()), calc_joltage_2(line.as_ref().unwrap()))).sum();
    println!("Result of part 1 is: {}, part 2 is: {}", totals.0, totals.1);
}
