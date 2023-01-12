use std::ops::Index;

use crate::types::Pos;

use super::{Move, Piece};

const INIT_BOARD: [[i8; 8]; 8] = [
    [4, 2, 3, 5, 6, 3, 2, 4],
    [1; 8],
    [0; 8],
    [0; 8],
    [0; 8],
    [0; 8],
    [-1; 8],
    [-4, -2, -3, -5, -6, -3, -2, -4],
];

pub struct Board([[i8; 8]; 8]);

impl Index<Pos> for Board {
    type Output = i8;
    fn index(&self, Pos(x, y): Pos) -> &Self::Output {
        &self.0[y as usize][x as usize]
    }
}

impl Board {
    fn new() -> Board {
        Board([
            [4, 2, 3, 5, 6, 3, 2, 4],
            [1; 8],
            [0; 8],
            [0; 8],
            [0; 8],
            [0; 8],
            [-1; 8],
            [-4, -2, -3, -5, -6, -3, -2, -4],
        ])
    }

    pub fn valid_moves(&self, player: i8) -> Vec<Move> {
        //let mut moves = Vec::new();
        todo!()
    }
}

const PIECE_SIZE: u8 = 3;
const PIECE_MASK: u8 = 0b111;

struct BitBoard(u8);

fn pos_to_shift(Pos(x, y): Pos) -> u8 {
    y as u8 * 8 * PIECE_SIZE + x as u8 * PIECE_SIZE
}

impl BitBoard {
    fn new() -> BitBoard {
        let mut board = 0_u8;
        for (y, row) in INIT_BOARD.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                board &=
                    ((piece + 6) as u8) << (y as u8 * 8 * PIECE_SIZE + x as u8 * PIECE_SIZE);
            }
        }
        BitBoard(board)
    }

    fn index(&self, index: Pos) -> i8 {
        ((self.0 >> pos_to_shift(index)) & PIECE_MASK) as i8 - 6
    }

    fn set(&mut self, index: Pos, piece: i8) {
        let shift = pos_to_shift(index);
        self.0 &= PIECE_MASK << shift;
        self.0 &= ((piece + 6) as u8) << shift;
    }
}
