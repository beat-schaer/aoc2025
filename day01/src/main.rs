mod dial;

use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<()> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut dial = dial::Dial::new(50);
    let mut part1_zero_count = 0;
    let mut part2_zero_count = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let direction = line.chars().next().ok_or(anyhow!("Line is empty"))?;
        let delta = i32::from_str(&line[direction.len_utf8()..])?;
        let (part1_res, part2_res) = match direction {
            'R' => dial.rotate_right(delta),
            'L' => dial.rotate_left(delta),
            _ => panic!("Unexpected direction found: {}", direction),
        };
        part1_zero_count += part1_res;
        part2_zero_count += part2_res;
    }
    println!(
        "*** Number of part1 zeros: {}, number of part2 zeros: {} ***",
        part1_zero_count, part2_zero_count
    );
    Ok(())
}
