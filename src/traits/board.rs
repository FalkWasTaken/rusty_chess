use std::ops::Index;

use crate::types::*;

pub trait Board: Index<Pos> {
    fn new() -> Self;
    fn valid_moves(&self) -> Vec<Move>;
    fn king_pos(&self) -> Pos;
    
}