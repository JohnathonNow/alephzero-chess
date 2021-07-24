use std::fmt;

use crate::piece::{Color, Piece, PieceType};
use num_bigint::BigInt;

const STANDARD_BOARD_SIZE: i32 = 8;

pub struct Board {
    turn: BigInt,
    white_can_castle: bool,
    black_can_castle: bool,
    pieces: Vec<Piece>,
}

impl Board {
    pub fn new() -> Self {
        let mut me = Self {
            turn: 0.into(),
            white_can_castle: true,
            black_can_castle: true,
            pieces: Vec::new(),
        };
        for x in 0..8 {
            me.add_piece_mirror(PieceType::Pawn(Color::White), 1, x);
        }
        me.add_piece_mirror_quad(PieceType::Rook(Color::White), 0, 0);
        me.add_piece_mirror_quad(PieceType::Knight(Color::White), 0, 1);
        me.add_piece_mirror_quad(PieceType::Bishop(Color::White), 0, 2);
        me.add_piece_mirror(PieceType::King(Color::White), 0, 3);
        me.add_piece_mirror(PieceType::Queen(Color::White), 0, 4);
        me
    }
    fn add_piece_mirror(&mut self, piece: PieceType, rank: i32, file: i32) {
        self.pieces
            .push(Piece::new(piece, rank.into(), file.into()));
        self.pieces.push(Piece::new(
            piece.flip(),
            (STANDARD_BOARD_SIZE - rank - 1).into(),
            file.into(),
        ));
    }
    fn add_piece_mirror_quad(&mut self, piece: PieceType, rank: i32, file: i32) {
        self.add_piece_mirror(piece, rank, file);
        self.add_piece_mirror(piece, rank, STANDARD_BOARD_SIZE - file - 1);
    }
    pub fn get_piece_at(&self, rank: BigInt, file: BigInt) -> Option<&Piece> {
        for p in &self.pieces {
            if p.rank == rank && p.file == file {
                return Some(&p);
            }
        }
        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in 0..STANDARD_BOARD_SIZE {
            for file in 0..STANDARD_BOARD_SIZE {
                if let Some(p) = self.get_piece_at(rank.into(), file.into()) {
                    p.piece.fmt(f)?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}