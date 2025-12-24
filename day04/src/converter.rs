use crate::def::{BoolLine, init_line};

pub fn convert_line<const SIZE: usize>(input: &str) -> BoolLine<SIZE>
{
    let mut line = init_line::<SIZE>();
    for n in 0..SIZE {
        line[n] = match input.chars().nth(n) {
            Some('.') => false,
            Some('@') => true,
            _ => panic!("Unexpected / missing char detected"),
        }
    }
    line
}
