pub fn calc_joltage_1(input: &str) -> u64
{
    let first_pos = find_first_highest_joltage_pos(&input[..input.len() - 1]);
    let second_pos = find_first_highest_joltage_pos(&input[first_pos + 1..]) + first_pos + 1;
    let joltage_str = format!("{}{}", input.chars().nth(first_pos).unwrap(), input.chars().nth(second_pos).unwrap());
    joltage_str.parse().unwrap()
}

pub fn calc_joltage_2(input: &str) -> u64
{
    let mut joltage_str = String::new();
    let mut left_offset: usize = 0;
    for i in 0..12 {
        let right_offset = 11 - i;
        let highest_digit_pos = find_first_highest_joltage_pos(&input[left_offset..input.len() - right_offset]) + left_offset;
        joltage_str += &format!("{}", input.chars().nth(highest_digit_pos).unwrap());
        left_offset = highest_digit_pos + 1;
    }
    joltage_str.parse().unwrap()
}

fn find_first_highest_joltage_pos(input: &str) -> usize
{
    let mut current_max_char = '0';
    let mut current_max_pos = 0;
    for (current_pos, current_char) in input.chars().enumerate() {
        if current_char > current_max_char {
            current_max_char = current_char;
            current_max_pos = current_pos;
        }
        if current_max_char == '9' {
            break;
        }
    }
    current_max_pos
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(calc_joltage_1("987654321111111"), 98);
        assert_eq!(calc_joltage_1("811111111111119"), 89);
        assert_eq!(calc_joltage_1("234234234234278"), 78);
        assert_eq!(calc_joltage_1("818181911112111"), 92);
    }

    #[test]
    fn test2()
    {
        assert_eq!(calc_joltage_2("987654321111111"), 987654321111);
        assert_eq!(calc_joltage_2("811111111111119"), 811111111119);
        assert_eq!(calc_joltage_2("234234234234278"), 434234234278);
        assert_eq!(calc_joltage_2("818181911112111"), 888911112111);
    }
}
