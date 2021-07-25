use std::fmt;

use crate::piece::{Color, Piece, PieceType};
use num_bigint::BigInt;
use num_traits::Signed;
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
    pub fn get_piece_at(&self, rank: &BigInt, file: &BigInt) -> Option<&Piece> {
        for p in &self.pieces {
            if &p.rank == rank && &p.file == file {
                return Some(&p);
            }
        }
        None
    }
    fn get_collision(
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
            let good_so_far = match p.piece {
                PieceType::Pawn(_) => self.is_pawn_move_legal(p, to_rank, to_file),
                PieceType::Knight(_) => self.is_knight_move_legal(p, to_rank, to_file),
                PieceType::Bishop(_) => self.is_bishop_move_legal(p, to_rank, to_file),
                PieceType::Rook(_) => self.is_rook_move_legal(p, to_rank, to_file),
                PieceType::Queen(_) => self.is_queen_move_legal(p, to_rank, to_file),
                PieceType::King(_) => self.is_king_move_legal(p, to_rank, to_file),
            };
            if let Some(other) = self.get_piece_at(to_rank, to_file) {
                if p.piece.get_color() == other.piece.get_color() {
                    return false;
                }
            }
            good_so_far
        } else {
            false //no piece there so not legal
        }
    }
    fn is_pawn_move_legal(&self, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
        if &p.file == to_file && to_rank == &(&p.rank + 1) { //single move
            if let None = self.get_piece_at(to_rank, to_file) {
                return true;
            }
        }
        if &p.file == to_file && to_rank == &(&p.rank + 2) && !p.has_moved { //double move at start
            if let None = self.get_piece_at(to_rank, to_file) {
                if let None = self.get_piece_at(&(&p.rank + 1), to_file) {
                    return true;
                }
            }
        }
        if (&p.file - to_file).abs() == 1.into() && to_rank == &(&p.rank + 1) { //take diagonally
            if let Some(_) = self.get_piece_at(to_rank, to_file) {
                return true;
            }
        }
        false
    }
    fn is_knight_move_legal(&self, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
        let df = &p.file - to_file;
        let dr = &p.rank - to_rank;
        if dr.abs() == 1.into() {
            (df.abs() - 2) % STANDARD_BOARD_SIZE == 0.into()
        } else if df.abs() == 1.into() {
            (dr.abs() - 2) % STANDARD_BOARD_SIZE == 0.into()
        } else {
            false
        }
    }
    fn is_rook_move_legal(&self, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
        if &p.file != to_file && to_rank != &p.rank {
            return false;
        }
        self.get_collision(&p.rank, &p.file, to_rank, to_file)
    }
    fn is_bishop_move_legal(&self, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
        if (&p.rank - to_rank).abs() != (&p.file - to_file).abs() {
            return false;
        }
        self.get_collision(&p.rank, &p.file, to_rank, to_file)
    }
    fn is_queen_move_legal(&self, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
        self.is_bishop_move_legal(p, to_rank, to_file)
            || self.is_rook_move_legal(p, to_rank, to_file)
    }
    fn is_king_move_legal(&self, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
        self.is_queen_move_legal(p, to_rank, to_file)
            && (&p.rank - to_rank).abs() <= 1.into()
            && (&p.file - to_file).abs() <= 1.into()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in 0..STANDARD_BOARD_SIZE {
            for file in 0..STANDARD_BOARD_SIZE {
                if let Some(p) = self.get_piece_at(&rank.into(), &file.into()) {
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

#[test]
fn test_pawns() {
    let b = Board::new();
    assert!(b.is_move_legal(&1.into(), &1.into(), &2.into(), &1.into()));
    assert!(b.is_move_legal(&1.into(), &1.into(), &3.into(), &1.into()));
    assert!(!b.is_move_legal(&1.into(), &1.into(), &4.into(), &1.into()));
}

#[test]
fn test_knights() {
    let b = Board::new();
    assert!(b.is_move_legal(&0.into(), &1.into(), &2.into(), &2.into()));
    assert!(b.is_move_legal(&0.into(), &6.into(), &2.into(), &7.into()));
    assert!(!b.is_move_legal(&0.into(), &1.into(), &4.into(), &1.into()));
}

#[test]
fn test_rooks() {
    let b = Board::new();
    assert!(b.is_move_legal(&0.into(), &0.into(), &(-4).into(), &0.into()));
    assert!(b.is_move_legal(&0.into(), &0.into(), &0.into(), &(-4).into()));
    assert!(!b.is_move_legal(&0.into(), &0.into(), &(-4).into(), &(-4).into()));
    assert!(!b.is_move_legal(&0.into(), &0.into(), &3.into(), &0.into()));
}
#[test]
fn test_bishops() {
    let b = Board::new();
    assert!(b.is_move_legal(&0.into(), &2.into(), &(-2).into(), &(0).into()));
    assert!(b.is_move_legal(&0.into(), &2.into(), &(-6).into(), &(-4).into()));
    assert!(!b.is_move_legal(&0.into(), &2.into(), &0.into(), &(-4).into()));
    assert!(!b.is_move_legal(&0.into(), &2.into(), &4.into(), &2.into()));
}