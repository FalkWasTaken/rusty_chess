#[derive(Clone, Copy)]
pub struct Pos(pub u8, pub u8);

impl From<(u8, u8)> for Pos {
    fn from((x, y): (u8, u8)) -> Self {
        Pos(x, y)
    }
}

impl From<(i8, i8)> for Pos {
    fn from((x, y): (i8, i8)) -> Self {
        Pos(x as u8, y as u8)
    }
}

pub type Move = (Pos, Pos);