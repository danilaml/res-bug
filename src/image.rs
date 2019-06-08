#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
pub struct Image {
    dim: (usize, usize),
    data: Vec<Color>,
}

