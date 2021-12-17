use std::fmt;
use serde::{Deserialize, Serialize};
use crate::pawn_rank::PawnRank;
use crate::piece::{Color, Piece};
use crate::piece_rules::{PieceRules, StandardChess};
use crate::piece_serializer::piece_serialize;
use num_bigint::BigInt;
use num_traits::Signed;
pub const STANDARD_BOARD_SIZE: i32 = 8;

//#[derive(Serialize, Deserialize)]
pub struct Board{
    pub turn: BigInt,
    pub white_can_castle: bool,
    pub black_can_castle: bool,
    pub pieces: Vec<Piece>,
    pub white_pawns: PawnRank,
    pub black_pawns: PawnRank,

}

impl Board {
    pub fn new() -> Self {
        Self {
            turn: 0.into(),
            white_can_castle: true,
            black_can_castle: true,
            pieces: Vec::new(),
            white_pawns: PawnRank::new(),
            black_pawns: PawnRank::new(),

                }
    }
    pub fn place_piece(&mut self, piece: Piece) {
        let x = piece;
        self.pieces.push(x);
    }
    pub fn do_move(&mut self, rank: &BigInt, file: &BigInt, to_rank: &BigInt, to_file: &BigInt) -> Option<usize> {
        let from = self.get_piece_at(rank, file)?;
        if let Some(to) = self.get_piece_at(to_rank, to_file) {
            self.pieces.get_mut(to).unwrap().capture();
        }
        let p = self.pieces.get_mut(from).unwrap();
        p.goto(to_rank, to_file);
        println!("{}", piece_serialize(p));
        None
    }
    pub fn get_piece_at(&mut self, rank: &BigInt, file: &BigInt) -> Option<usize> {
        if *rank == 1.into() && !self.black_pawns.has_moved(file) {
            self.black_pawns.set_moved(file);
            self.place_piece(Piece::new("pawn".to_string(), Color::Black, rank.clone(), file.clone()));
        }
        if *rank == 6.into() && !self.white_pawns.has_moved(file) {
            self.white_pawns.set_moved(file);
            self.place_piece(Piece::new("pawn".to_string(), Color::White, rank.clone(), file.clone()));

        }
        for i in 0..self.pieces.len() {
            if self.pieces[i].get_rank() == rank && self.pieces[i].get_file() == file && !self.pieces[i].is_captured() {
                return Some(i);
            }
        }
        None
    }
    pub fn get_collision(
        &mut self,
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
    pub fn is_move_legal<T: PieceRules>(
        s: &mut Board,
        rules: &T,
        from_rank: &BigInt,
        from_file: &BigInt,
        to_rank: &BigInt,
        to_file: &BigInt,
    ) -> bool {
        if from_rank == to_rank && from_file == to_file {
            return false;
        }
        if let Some(p) = s.get_piece_at(from_rank, from_file) {
            let good_so_far = rules.can_move( s, p, to_rank, to_file);
            if let Some(other) = s.get_piece_at(to_rank, to_file) {
                if s.pieces[p].get_color() == s.pieces[other].get_color() {
                    return false;
                }
            }
            good_so_far
        } else {
            false //no piece there so not legal
        }
    }
}

