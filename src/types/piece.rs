pub const PAWN: i8 = 1;
pub const KNIGHT: i8 = 2;
pub const BISHOP: i8 = 3;
pub const ROOK: i8 = 4;
pub const QUEEN: i8 = 5;
pub const KING: i8 = 6;

pub enum Type {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6
}

impl From<i8> for Type {
    fn from(value: i8) -> Self {
        match value {
            1 => Type::Pawn,
            2 => Type::Knight,
            3 => Type::Bishop,
            4 => Type::Rook,
            5 => Type::Queen,
            6 => Type::King,
            _ => panic!("Invalid piece type! Expected range (1..=6), got {value}")
        }
    }
}

pub enum Piece {
    White(Type),
    Black(Type),
    Empty
}

impl From<i8> for Piece {
    fn from(value: i8) -> Self {
        match value {
            0 => Piece::Empty,
            1.. => Piece::White(value.into()),
            _ => Piece::Black(value.abs().into())

        }
    }
}