mod product_id;

use std::fs;
use product_id::{check_number_range_1, check_number_range_2, ProductNbr, ProductNbrColl};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let content = fs::read_to_string("data/input.txt")?;
    let ranges = parse_product_nbr_ranges(&content)?;
    let invalid_numbers_1: ProductNbrColl = ranges.iter().flat_map(|range| check_number_range_1(range.0, range.1)).collect();
    let invalid_numbers_2: ProductNbrColl = ranges.iter().flat_map(|range| check_number_range_2(range.0, range.1)).collect();
    let result_1: ProductNbr = invalid_numbers_1.iter().sum();
    let result_2: ProductNbr = invalid_numbers_2.iter().sum();
    println!("*** Result for part 1 is {} and for part 2 is {} ***", result_1, result_2);
    Ok(())
}
 

fn parse_product_nbr_ranges(input: &str) -> Result<Vec<(ProductNbr, ProductNbr)>> {
    let ranges_str : Vec<&str> = input.split(',').collect();
    ranges_str.iter().map(|input| parse_product_nbr_range(input)).collect()
}

fn parse_product_nbr_range(input: &str) -> Result<(ProductNbr, ProductNbr)> {
    let parts: Vec<&str> = input.trim().split('-').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid range format: '{}'", input));
    }
    let start: ProductNbr = parts[0].parse()
        .map_err(|_| anyhow!("Invalid number in range: '{}'", input))?;
    let end: ProductNbr = parts[1].parse()
        .map_err(|_| anyhow!("Invalid number in range: '{}'", input))?;
    if start > end {
        return Err(anyhow!("Range start > end in '{}'", input));
    }
    Ok((start, end))
}