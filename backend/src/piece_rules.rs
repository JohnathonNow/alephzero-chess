use std::collections::HashMap;

use num_bigint::BigInt;
use num_traits::Signed;

#[cfg(test)]
use crate::piece::Piece;
use crate::{
    board::{Board, STANDARD_BOARD_SIZE},
    piece::Color, moves::Move, piece_serializer,
};

pub trait PieceRules {
    fn can_move(&self, board: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Option<Move>;
}

#[derive(Clone)]
pub struct StandardChess {
    map: HashMap<String, fn(&mut Board, usize, &BigInt, &BigInt) -> Option<Move>>,
}

fn maybe_capture(b: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Move {
    let move_ = Move::new(i);
    if let Some(x) = b.get_piece_at(to_rank, to_file) {
        return move_.add_capture(x);
    }
    move_
}

impl StandardChess {
    pub fn new() -> Self {
        let map = HashMap::new();
        let mut me = Self { map };
        me.add_piecetype("pawn".into(), is_pawn_move_legal);
        me.add_piecetype("knight".into(), is_knight_move_legal);
        me.add_piecetype("bishop".into(), is_bishop_move_legal);
        me.add_piecetype("rook".into(), is_rook_move_legal);
        me.add_piecetype("queen".into(), is_queen_move_legal);
        me.add_piecetype("king".into(), is_king_move_legal);

        me
    }
    fn add_piecetype(&mut self, name: String, pt: fn(&mut Board, usize, &BigInt, &BigInt) -> Option<Move>) {
        self.map.insert(name, pt);
    }
    #[cfg(test)]
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
fn is_pawn_move_legal(board: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Option<Move> {
    let dir = if board.pieces[i].get_color() == Color::Black {
        1
    } else {
        -1
    };
    if board.pieces[i].get_file() == to_file && to_rank == &(board.pieces[i].get_rank() + dir) {
        //single move
        if let None = board.get_piece_at(to_rank, to_file) {
            return Some(Move::standard(i, to_rank, to_file));
        }
    }
    if board.pieces[i].get_file() == to_file
        && to_rank == &(board.pieces[i].get_rank() + dir * 2)
        && !board.pieces[i].has_moved()
    {
        //double move at start
        if let None = board.get_piece_at(to_rank, to_file) {
            if let None = board.get_piece_at(&(board.pieces[i].get_rank() + dir), to_file) {
                return Some(Move::standard(i, to_rank, to_file));
            }
        }
    }
    if (board.pieces[i].get_file() - to_file).abs() == 1.into()
        && to_rank == &(board.pieces[i].get_rank() + dir)
    {
        //take diagonally
        if let Some(x) = board.get_piece_at(to_rank, to_file) {
            return Some(Move::capture(i, to_rank, to_file, x));
        }
        let rank = board.pieces[i].get_rank().clone(); //bad
                                                       //en passant!
        if let Some(p) = board.get_piece_at(&rank, to_file) {
            if board.pieces[p].get_type() == "pawn"
                && board.pieces[p].get_color() != board.pieces[i].get_color()
                && board.last_move().map_or(false, |x| x == p) {
                    return Some(Move::capture(i, to_rank, to_file, p))
                }
        }
    }
    None
}
fn is_knight_move_legal(board: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Option<Move> {
    let p = board.pieces.get(i).unwrap();
    let df = p.get_file() - to_file;
    let dr = p.get_rank() - to_rank;
    let mut move_ = maybe_capture(board, i, to_rank, to_file);
    if dr.abs() == 1.into() {
        if (df.abs() - 2) % STANDARD_BOARD_SIZE == 0.into() {
            return Some(move_.add_motion(i, to_rank, to_file));
        }
    } else if df.abs() == 1.into() {
        if (dr.abs() - 2) % STANDARD_BOARD_SIZE == 0.into() {
            return Some(move_.add_motion(i, to_rank, to_file));
        }
    }
    None 
}  
fn is_rook_move_legal(board: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Option<Move> {
    if board.pieces[i].get_file() != to_file && to_rank != board.pieces[i].get_rank() {
        return None;
    }
    let mut move_ = maybe_capture(board, i, to_rank, to_file);
    if board.get_collision(
        &board.pieces[i].get_rank().clone(),
        &board.pieces[i].get_file().clone(),
        to_rank,
        to_file,
    ) {
        Some(move_.add_motion(i, to_rank, to_file))
    } else {
        None
    }
}
fn is_bishop_move_legal(board: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Option<Move> {
    if (board.pieces[i].get_rank() - to_rank).abs() != (board.pieces[i].get_file() - to_file).abs()
    {
        return None;
    }
    let mut move_ = maybe_capture(board, i, to_rank, to_file);
    if board.get_collision(
        &board.pieces[i].get_rank().clone(),
        &board.pieces[i].get_file().clone(),
        to_rank,
        to_file,
    ) {
        Some(move_.add_motion(i, to_rank, to_file))
    } else {
        None
    }
}
fn is_queen_move_legal(board: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Option<Move> {
    is_bishop_move_legal(board, i, to_rank, to_file).or(is_rook_move_legal(board, i, to_rank, to_file))
}
fn is_king_move_legal(board: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Option<Move> {
    let qm = is_queen_move_legal(board, i, to_rank, to_file);
    if qm.is_some()
        && (board.pieces[i].get_rank() - to_rank).abs() <= 1.into()
        && (board.pieces[i].get_file() - to_file).abs() <= 1.into()
    {
        return qm;
    }
    // Castling
    if board.pieces[i].has_moved() {
        return None;
    }
    if to_rank != board.pieces[i].get_rank() {
        return None;
    }
    if to_file == &6.into() {
        if let Some(z) = board.get_piece_at(to_rank, &7.into()) {
            if !board.pieces[z].has_moved()
                && board.get_piece_at(to_rank, &6.into()).is_none()
                && board.get_piece_at(to_rank, &5.into()).is_none() {
                    return Some(Move::standard(i, to_rank, to_file).add_motion(z, to_rank, &5.into()))
                }
        }
    }
    if to_file == &2.into() {
        if let Some(z) = board.get_piece_at(to_rank, &0.into()) {
            if !board.pieces[z].has_moved()
                && board.get_piece_at(to_rank, &1.into()).is_none()
                && board.get_piece_at(to_rank, &2.into()).is_none()
                && board.get_piece_at(to_rank, &3.into()).is_none() {
                    return Some(Move::standard(i, to_rank, to_file).add_motion(z, to_rank, &3.into()))
                }
        }
    }
    None
}
#[test]
fn test_pawns() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    b.place_piece(
        pm.build_piece(&"pawn".into(), Color::Black, 1.into(), 1.into())
            .unwrap(),
    );

    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &1.into(),
        &1.into(),
        &2.into(),
        &1.into()
    ));
    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &1.into(),
        &1.into(),
        &3.into(),
        &1.into()
    ));
    assert!(!Board::is_move_legal(
        &mut b,
        &pm,
        &1.into(),
        &1.into(),
        &4.into(),
        &1.into()
    ));
}

#[test]
fn test_knights() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    b.place_piece(
        pm.build_piece(&"knight".into(), Color::White, 0.into(), 1.into())
            .unwrap(),
    );
    b.place_piece(
        pm.build_piece(&"knight".into(), Color::White, 0.into(), 6.into())
            .unwrap(),
    );

    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &1.into(),
        &2.into(),
        &2.into()
    ));
    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &6.into(),
        &2.into(),
        &7.into()
    ));
    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &1.into(),
        &(-1).into(),
        &(-1).into()
    ));
}

#[test]
fn test_rooks() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    b.place_piece(
        pm.build_piece(&"rook".into(), Color::White, 0.into(), 0.into())
            .unwrap(),
    );
    b.place_piece(
        pm.build_piece(&"pawn".into(), Color::White, 1.into(), 0.into())
            .unwrap(),
    );

    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &0.into(),
        &(-4).into(),
        &0.into()
    ));
    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &0.into(),
        &0.into(),
        &(-4).into()
    ));
    assert!(!Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &0.into(),
        &(-4).into(),
        &(-4).into()
    ));
    assert!(!Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &0.into(),
        &3.into(),
        &0.into()
    ));
}
#[test]
fn test_bishops() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    b.place_piece(
        pm.build_piece(&"bishop".into(), Color::White, 0.into(), 2.into())
            .unwrap(),
    );

    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &2.into(),
        &(-2).into(),
        &(0).into()
    ));
    assert!(Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &2.into(),
        &(-6).into(),
        &(-4).into()
    ));
    assert!(!Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &2.into(),
        &0.into(),
        &(-4).into()
    ));
    assert!(!Board::is_move_legal(
        &mut b,
        &pm,
        &0.into(),
        &2.into(),
        &4.into(),
        &2.into()
    ));
}

#[test]
fn test_en_passant() {
    let mut b = Board::new();
    let pm = StandardChess::new();
    let m = Board::move_legal(&mut b, &pm, &1.into(), &3.into(), &3.into(), &3.into());
    b.do_move(m.unwrap());
    b.place_piece(
        pm.build_piece(&"pawn".into(), Color::White, 3.into(), 4.into())
            .unwrap(),
    );
    assert!(is_pawn_move_legal(&mut b, 1, &2.into(), &3.into()).is_some())
}

impl PieceRules for StandardChess {
    fn can_move(&self, board: &mut Board, i: usize, to_rank: &BigInt, to_file: &BigInt) -> Option<Move> {
        let piece = board.pieces.get(i).unwrap();
        self.map.get(piece.get_type()).unwrap()(board, i, to_rank, to_file)
    }
}
