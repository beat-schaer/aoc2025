pub struct Dial {
    current_pos: i32,
}

impl Dial {
    pub fn new(initial_pos: i32) -> Self {
        Self {
            current_pos: initial_pos,
        }
    }

    fn get_part1_zero_clicks(self: &Self) -> i32 {
        match self.current_pos {
            0 => 1,
            _ => 0,
        }
    }

    pub fn rotate_right(self: &mut Self, delta: i32) -> (i32, i32) {
        let mut part2_zero_clicks = delta / 100;
        let rest_delta = delta % 100;
        for _ in 0..rest_delta {
            self.current_pos += 1;
            if self.current_pos == 100 {
                self.current_pos = 0;
                part2_zero_clicks += 1;
            }
        }
        (self.get_part1_zero_clicks(), part2_zero_clicks)
    }

    pub fn rotate_left(self: &mut Self, delta: i32) -> (i32, i32) {
        let mut part2_zero_clicks = delta / 100;
        let rest_delta = delta % 100;
        for _ in 0..rest_delta {
            self.current_pos -= 1;
            if self.current_pos == 0 {
                part2_zero_clicks += 1;
            }
            if self.current_pos == -1 {
                self.current_pos = 99;
            }
        }
        (self.get_part1_zero_clicks(), part2_zero_clicks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut dial = Dial::new(50);
        perform_left_test_part1(&mut dial, 68, 0, 82);
        perform_left_test_part1(&mut dial, 30, 0, 52);
        perform_right_test_part1(&mut dial, 48, 1, 0);
        perform_left_test_part1(&mut dial, 5, 0, 95);
        perform_right_test_part1(&mut dial, 60, 0, 55);
        perform_left_test_part1(&mut dial, 55, 1, 0);
        perform_left_test_part1(&mut dial, 1, 0, 99);
        perform_left_test_part1(&mut dial, 99, 1, 0);
        perform_right_test_part1(&mut dial, 14, 0, 14);
        perform_left_test_part1(&mut dial, 82, 0, 32);
    }

    fn perform_right_test_part1(dial: &mut Dial, delta: i32, expected_clicks: i32, new_pos: i32) {
        let (part1_clicks, _) = dial.rotate_right(delta);
        assert_eq!(
            part1_clicks, expected_clicks,
            "part1_clicks == expected_clicks"
        );
        assert_eq!(dial.current_pos, new_pos, "dial.current_pos == new_pos");
    }

    fn perform_left_test_part1(dial: &mut Dial, delta: i32, expected_clicks: i32, new_pos: i32) {
        let (part1_clicks, _) = dial.rotate_left(delta);
        assert_eq!(
            part1_clicks, expected_clicks,
            "part1_clicks == expected_clicks"
        );
        assert_eq!(dial.current_pos, new_pos, "dial.current_pos == new_pos");
    }

    #[test]
    fn test_part2() {
        let mut dial = Dial::new(50);
        perform_left_test_part2(&mut dial, 68, 1, 82);
        perform_left_test_part2(&mut dial, 30, 0, 52);
        perform_right_test_part2(&mut dial, 48, 1, 0);
        perform_left_test_part2(&mut dial, 5, 0, 95);
        perform_right_test_part2(&mut dial, 60, 1, 55);
        perform_left_test_part2(&mut dial, 55, 1, 0);
        perform_left_test_part2(&mut dial, 1, 0, 99);
        perform_left_test_part2(&mut dial, 99, 1, 0);
        perform_right_test_part2(&mut dial, 14, 0, 14);
        perform_left_test_part2(&mut dial, 82, 1, 32);
    }

    fn perform_right_test_part2(dial: &mut Dial, delta: i32, expected_clicks: i32, new_pos: i32) {
        let (_, part2_clicks) = dial.rotate_right(delta);
        println!(
            "turning right {:>2}\tgot {} clicks\tnew position is {:>2}",
            delta, part2_clicks, dial.current_pos
        );
        assert_eq!(
            part2_clicks, expected_clicks,
            "part2_clicks == expected_clicks"
        );
        assert_eq!(dial.current_pos, new_pos, "dial.current_pos == new_pos");
    }

    fn perform_left_test_part2(dial: &mut Dial, delta: i32, expected_clicks: i32, new_pos: i32) {
        let (_, part2_clicks) = dial.rotate_left(delta);
        println!(
            "turning left  {:>2}\tgot {} clicks\tnew position is {:>2}",
            delta, part2_clicks, dial.current_pos
        );
        assert_eq!(
            part2_clicks, expected_clicks,
            "part2_clicks == expected_clicks"
        );
        assert_eq!(dial.current_pos, new_pos, "dial.current_pos == new_pos");
    }
}
