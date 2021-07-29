use std::fmt;

use crate::piece::{Color, Piece, PieceType};
use num_bigint::BigInt;
use num_traits::Signed;
pub const STANDARD_BOARD_SIZE: i32 = 8;

pub struct Board {
    turn: BigInt,
    white_can_castle: bool,
    black_can_castle: bool,
    pieces: Vec<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            turn: 0.into(),
            white_can_castle: true,
            black_can_castle: true,
            pieces: Vec::new(),
        }
    }
    pub fn place_piece(&mut self, piece: Piece) {
        let x = piece;
        self.pieces.push(x);
    }
    pub fn get_piece_at(&self, rank: &BigInt, file: &BigInt) -> Option<&Piece> {
        for p in &self.pieces {
            if p.get_rank() == rank && p.get_file() == file {
                return Some(&p);
            }
        }
        None
    }
    pub fn get_collision(
        &self,
        from_rank: &BigInt,
        from_file: &BigInt,
        to_rank: &BigInt,
        to_file: &BigInt,
    ) -> bool {
        let dr = match (to_rank - from_rank).sign() {
            num_bigint::Sign::Minus => -1,
            num_bigint::Sign::NoSign => 0,
            num_bigint::Sign::Plus => 1,
        };
        let df = match (to_file - from_file).sign() {
            num_bigint::Sign::Minus => -1,
            num_bigint::Sign::NoSign => 0,
            num_bigint::Sign::Plus => 1,
        };
        let mut rank = from_rank.clone();
        let mut file = from_file.clone();
        rank += dr;
        file += df;
        while &rank != to_rank || &file != to_file {
            if let Some(_) = self.get_piece_at(&rank, &file) {
                return false;
            }
            rank += dr;
            file += df;
        }
        true
    }
    pub fn is_move_legal(
        &self,
        from_rank: &BigInt,
        from_file: &BigInt,
        to_rank: &BigInt,
        to_file: &BigInt,
    ) -> bool {
        if from_rank == to_rank && from_file == to_file {
            return false;
        }
        if let Some(p) = self.get_piece_at(from_rank, from_file) {
            let good_so_far = p.can_move(&self, to_rank, to_file);
            if let Some(other) = self.get_piece_at(to_rank, to_file) {
                if p.get_color() == other.get_color() {
                    return false;
                }
            }
            good_so_far
        } else {
            false //no piece there so not legal
        }
    }
}
