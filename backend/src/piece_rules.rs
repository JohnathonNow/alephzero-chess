use std::collections::HashMap;

use num_bigint::BigInt;
use num_traits::Signed;

use crate::{
    board::{Board, STANDARD_BOARD_SIZE},
    piece::{Color, Piece},
};

pub trait PieceRules {
    fn can_move(&self, board: &Board, piece: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool;
}

#[derive(Clone)]
pub struct StandardChess {
    map: HashMap<String, fn(&Board, &Piece, &BigInt, &BigInt) -> bool>,
}

impl StandardChess {
    pub fn new() -> Self {
        let map = HashMap::new();
        let mut me = Self { map };
        me.add_piecetype("pawn".into(), (is_pawn_move_legal));
        me.add_piecetype("knight".into(), (is_knight_move_legal));
        me.add_piecetype("bishop".into(), (is_bishop_move_legal));
        me.add_piecetype("rook".into(), (is_rook_move_legal));
        me.add_piecetype("queen".into(), (is_queen_move_legal));
        me.add_piecetype("king".into(), (is_king_move_legal));

        me
    }
    fn add_piecetype(&mut self, name: String, pt: fn(&Board, &Piece, &BigInt, &BigInt) -> bool) {
        self.map.insert(name, pt);
    }
    fn build_piece(
        &self,
        piece: &String,
        color: Color,
        rank: BigInt,
        file: BigInt,
    ) -> Option<Piece> {
        Some(Piece::new(piece.clone(), color, rank, file))
    }

}
fn is_pawn_move_legal(board: &Board, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
    if p.get_file() == to_file && to_rank == &(p.get_rank() + 1) {
        //single move
        if let None = board.get_piece_at(to_rank, to_file) {
            return true;
        }
    }
    if p.get_file() == to_file && to_rank == &(p.get_rank() + 2) && !p.has_moved() {
        //double move at start
        if let None = board.get_piece_at(to_rank, to_file) {
            if let None = board.get_piece_at(&(p.get_rank() + 1), to_file) {
                return true;
            }
        }
    }
    if (p.get_file() - to_file).abs() == 1.into() && to_rank == &(p.get_rank() + 1) {
        //take diagonally
        if let Some(_) = board.get_piece_at(to_rank, to_file) {
            return true;
        }
    }
    false
}
fn is_knight_move_legal(board: &Board, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
    let df = p.get_file() - to_file;
    let dr = p.get_rank() - to_rank;
    if dr.abs() == 1.into() {
        (df.abs() - 2) % STANDARD_BOARD_SIZE == 0.into()
    } else if df.abs() == 1.into() {
        (dr.abs() - 2) % STANDARD_BOARD_SIZE == 0.into()
    } else {
        false
    }
}
fn is_rook_move_legal(board: &Board, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
    if p.get_file() != to_file && to_rank != p.get_rank() {
        return false;
    }
    board.get_collision(p.get_rank(), p.get_file(), to_rank, to_file)
}
fn is_bishop_move_legal(board: &Board, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
    if (p.get_rank() - to_rank).abs() != (p.get_file() - to_file).abs() {
        return false;
    }
    board.get_collision(p.get_rank(), p.get_file(), to_rank, to_file)
}
fn is_queen_move_legal(board: &Board, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
    is_bishop_move_legal(board, p, to_rank, to_file)
        || is_rook_move_legal(board, p, to_rank, to_file)
}
fn is_king_move_legal(board: &Board, p: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
    is_queen_move_legal(board, p, to_rank, to_file)
        && (p.get_rank() - to_rank).abs() <= 1.into()
        && (p.get_file() - to_file).abs() <= 1.into()
}
#[test]
fn test_pawns() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    b.place_piece(pm.build_piece(&"pawn".into(), Color::White, 1.into(), 1.into()).unwrap());

    assert!(b.is_move_legal(&pm, &1.into(), &1.into(), &2.into(), &1.into()));
    assert!(b.is_move_legal(&pm, &1.into(), &1.into(), &3.into(), &1.into()));
    assert!(!b.is_move_legal(&pm, &1.into(), &1.into(), &4.into(), &1.into()));
}

#[test]
fn test_knights() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    b.place_piece(pm.build_piece(&"knight".into(), Color::White, 0.into(), 1.into()).unwrap());
    b.place_piece(pm.build_piece(&"knight".into(), Color::White, 0.into(), 6.into()).unwrap());

    assert!(b.is_move_legal(&pm, &0.into(), &1.into(), &2.into(), &2.into()));
    assert!(b.is_move_legal(&pm, &0.into(), &6.into(), &2.into(), &7.into()));
    assert!(!b.is_move_legal(&pm, &0.into(), &1.into(), &4.into(), &1.into()));
}

#[test]
fn test_rooks() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    b.place_piece(pm.build_piece(&"rook".into(), Color::White, 0.into(), 0.into()).unwrap());
    b.place_piece(pm.build_piece(&"pawn".into(), Color::White, 1.into(), 0.into()).unwrap());

    assert!(b.is_move_legal(&pm, &0.into(), &0.into(), &(-4).into(), &0.into()));
    assert!(b.is_move_legal(&pm, &0.into(), &0.into(), &0.into(), &(-4).into()));
    assert!(!b.is_move_legal(&pm, &0.into(), &0.into(), &(-4).into(), &(-4).into()));
    assert!(!b.is_move_legal(&pm, &0.into(), &0.into(), &3.into(), &0.into()));
}
#[test]
fn test_bishops() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    b.place_piece(pm.build_piece(&"bishop".into(), Color::White, 0.into(), 2.into()).unwrap());

    assert!(b.is_move_legal(&pm, &0.into(), &2.into(), &(-2).into(), &(0).into()));
    assert!(b.is_move_legal(&pm, &0.into(), &2.into(), &(-6).into(), &(-4).into()));
    assert!(!b.is_move_legal(&pm, &0.into(), &2.into(), &0.into(), &(-4).into()));
    assert!(!b.is_move_legal(&pm, &0.into(), &2.into(), &4.into(), &2.into()));
}

impl PieceRules for StandardChess {
    fn can_move(&self, board: &Board, piece: &Piece, to_rank: &BigInt, to_file: &BigInt) -> bool {
        self.map.get(piece.get_type()).unwrap()(board, piece, to_rank, to_file)
    }
}