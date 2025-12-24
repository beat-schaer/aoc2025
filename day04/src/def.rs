pub type BoolLine<const SIZE: usize> = [bool; SIZE];
pub type BoolMatrix<const SIZE: usize> = [BoolLine<SIZE>; SIZE];

pub const fn init_line<const SIZE: usize>() -> BoolLine<SIZE>
{
    [false; SIZE]
}

pub const fn init_matrix<const SIZE: usize>() -> BoolMatrix<SIZE>
{
    [init_line(); SIZE]
}
