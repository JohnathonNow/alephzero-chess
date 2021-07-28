use std::{collections::HashMap, fmt};

use num_bigint::BigInt;

use crate::board::Board;
#[derive(Clone, Copy)]
pub struct PieceType {
    _can_move: fn(&Board, &Piece, &BigInt, &BigInt) -> bool,
}

impl PieceType {
    fn can_move(&self, board: &Board, piece: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
        (self._can_move)(board, piece, to_rank, to_file)
    }
}

#[derive(Clone)]
struct PieceManager {
    map: HashMap<String, PieceType>,
}

impl PieceManager {
    fn new(map: HashMap<String, PieceType>) -> Self { Self { map } }
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn flip(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Clone)]
pub struct Piece {
    piece: PieceType,
    rank: BigInt,
    file: BigInt,
    captured: bool,
    has_moved: bool,
    color: Color,
}

impl Piece {
    pub fn new(piece: PieceType, color: Color, rank: BigInt, file: BigInt) -> Self {
        Self {
            piece,
            color,
            rank,
            file,
            captured: false,
            has_moved: false,
        }
    }
    pub fn can_move(&self, board: &Board, to_rank: &BigInt, to_file: &BigInt) -> bool {
        self.piece.can_move(board, self, to_rank, to_file)
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_rank(&self) -> &BigInt {
        &self.rank
    }
    pub fn get_file(&self) -> &BigInt {
        &self.file
    }
}
