pub struct Dial {
    current_pos: i32,
}

impl Dial {
    pub fn new(initial_pos: i32) -> Self {
        Self {
            current_pos: initial_pos,
        }
    }

    fn perform_rotation<AddOrSub, Checker>(
        self: &mut Self,
        add_or_sub: AddOrSub,
        while_checker: Checker,
        delta: i32,
    ) -> i32
    where
        AddOrSub: Fn(i32, i32) -> i32,
        Checker: Fn(i32) -> bool,
    {
        let mut rest_delta = delta;
        let mut zero_clicks = 0;
        while while_checker(add_or_sub(self.current_pos, rest_delta)) {
            rest_delta -= 100;
            zero_clicks += 1;
        }
        self.current_pos = add_or_sub(self.current_pos, rest_delta);
        if self.current_pos == 0 {
            zero_clicks += 1;
        }
        println!(
            "\tzero-clicks: {:>2}\tnew position: {:>2}",
            zero_clicks, self.current_pos
        );
        zero_clicks
    }

    pub fn rotate_right(self: &mut Self, delta: i32) -> i32 {
        print!("Rotate right {:>3} clicks.", delta);
        self.perform_rotation(|a, b| a + b, |d| d >= 100, delta)
    }

    pub fn rotate_left(self: &mut Self, delta: i32) -> i32 {
        print!("Rotate left  {:>3} clicks.", delta);
        self.perform_rotation(|a, b| a - b, |d| d < 0, delta)
    }
}
