use crate::board_serializer::{board_deserialize, board_serialize};
use crate::moves::Move;
use crate::pawn_rank::PawnRank;
use crate::piece::{Color, Piece};
use crate::piece_rules::{self, PieceRules, StandardChess};
use crate::piece_serializer::piece_serialize;
use num_bigint::BigInt;
use num_traits::Signed;
use wasm_bindgen::prelude::*;
pub const STANDARD_BOARD_SIZE: i32 = 8;
use crate::ai;
//#[derive(Serialize, Deserialize)]

#[wasm_bindgen]
pub struct WasmBoard {
    pub(crate) board: Board,
    pub(crate) rules: StandardChess,
}

#[wasm_bindgen]
impl WasmBoard {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            rules: StandardChess::new(),
        }
    }
    pub fn ai(&mut self) -> bool {
        if let Some(m) = ai::ai(&mut self.board, &mut self.rules) {
            self.board.do_move(m);
            false
        } else {
            true
        }
    }
    pub fn build(&mut self, s: String) {
        board_deserialize(&mut self.board, &s);
    }
    pub fn deconstruct(&self) -> String {
        board_serialize(&self.board)
    }
    pub fn place_piece(
        &mut self,
        piece: String,
        white: bool,
        rank: String,
        file: String,
    ) -> Option<usize> {
        self.board.place_piece(Piece::new(
            piece,
            if white { Color::White } else { Color::Black },
            rank.parse::<BigInt>().ok()?,
            file.parse::<BigInt>().ok()?,
        ))
    }
    pub fn get_piece_at(&mut self, rank: String, file: String) -> Option<usize> {
        self.board
            .get_piece_at(&rank.parse::<BigInt>().ok()?, &file.parse::<BigInt>().ok()?)
    }
    pub fn add_pawns(&mut self, file: String, zoom: String) -> Option<usize> {
        let f = file.parse::<BigInt>().ok()?;
        let mut z = zoom.parse::<BigInt>().ok()?;
        let zero = 0.into();
        while z >= zero {
            z -= &1.into();
            self.board.get_piece_at(&1.into(), &(&f + &z)); //not great, improve later
            self.board.get_piece_at(&6.into(), &(&f + &z));
        }
        Some(0)
    }
    pub fn get_piece_info(&mut self, id: usize) -> Option<String> {
        Some(piece_serialize(self.board.pieces.get(id)?))
    }
    pub fn promote(&mut self, rank: String, file: String, new_type: String) -> Option<usize> {
        self.board.promote(
            &rank.parse::<BigInt>().ok()?,
            &file.parse::<BigInt>().ok()?,
            new_type,
        )
    }
    pub fn do_move(
        &mut self,
        rank: String,
        file: String,
        to_rank: String,
        to_file: String,
    ) -> Option<usize> {
        let m = Board::move_legal(
            &mut self.board,
            &self.rules,
            &rank.parse::<BigInt>().ok()?,
            &file.parse::<BigInt>().ok()?,
            &to_rank.parse::<BigInt>().ok()?,
            &to_file.parse::<BigInt>().ok()?,
        )?;
        self.board.do_move(m)
    }
    pub fn undo_move(&mut self) -> Option<usize> {
        self.board.undo_move()
    }
    pub fn is_move_legal(
        &mut self,
        rank: String,
        file: String,
        to_rank: String,
        to_file: String,
    ) -> Option<bool> {
        Some(Board::is_move_legal(
            &mut self.board,
            &self.rules,
            &rank.parse::<BigInt>().ok()?,
            &file.parse::<BigInt>().ok()?,
            &to_rank.parse::<BigInt>().ok()?,
            &to_file.parse::<BigInt>().ok()?,
        ))
    }
    pub fn is_checkmate(&mut self) -> bool {
        let c = if self.board.moves.len() % 2 == 0 {
            Color::White
        } else {
            Color::Black
        };
        self.rules.is_checkmate(&mut self.board, c)
    }
    pub fn get_pieces(&mut self) -> Option<String> {
        let mut s = Vec::new();
        for piece in &self.board.pieces {
            if !piece.is_captured() {
                s.push(piece_serialize(piece));
            }
        }
        Some(format!("[{}]", s.join(",")))
    }

    pub fn get_legal_moves(
        &mut self,
        srank: String,
        sfile: String,
        swinx: String,
        swiny: String,
        szoom: String,
    ) -> Option<String> {
        let rank = srank.parse::<BigInt>().ok()?;
        let file = sfile.parse::<BigInt>().ok()?;
        let winx = swinx.parse::<BigInt>().ok()?;
        let winy = swiny.parse::<BigInt>().ok()?;
        let zoom = szoom.parse::<BigInt>().ok()?;
        let mut xx = winx.clone();
        let winxwidth = winx + zoom.clone();
        let winyheight = winy.clone() + zoom;
        let mut results = Vec::new();
        while xx < winxwidth {
            let mut yy = winy.clone();
            while yy < winyheight {
                if Board::is_move_legal(&mut self.board, &self.rules, &rank, &file, &xx, &yy) {
                    results.push(format!("[{}, {}]", xx, yy));
                }
                yy += 1;
            }
            xx += 1;
        }
        Some(format!("[{}]", results.join(",")))
    }
}

#[derive(Clone)]
pub struct Board {
    pub(crate) turn: BigInt,
    pub(crate) pieces: Vec<Piece>,
    pub(crate) white_pawns: PawnRank,
    pub(crate) black_pawns: PawnRank,
    pub(crate) moves: Vec<Move>,
    pub(crate) white_king: usize,
    pub(crate) black_king: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            turn: 0.into(),
            pieces: Vec::new(),
            white_pawns: PawnRank::new(),
            black_pawns: PawnRank::new(),
            moves: Vec::new(),
            black_king: 0,
            white_king: 0,
        }
    }
    pub fn promote(&mut self, rank: &BigInt, file: &BigInt, new_type: String) -> Option<usize> {
        self.get_piece_at(rank, file).and_then(|i| {
            self.pieces[i].set_type(new_type);
            Some(i)
        })
    }
    pub(crate) fn place_piece(&mut self, piece: Piece) -> Option<usize> {
        let x = piece;
        if x.get_type() == "king" {
            if x.get_color() == Color::White {
                self.white_king = self.pieces.len();
            } else {
                self.black_king = self.pieces.len();
            }
        }
        self.pieces.push(x);
        Some(self.pieces.len() - 1)
    }
    pub(crate) fn get_last_move(&self) -> Option<&Move> {
        self.moves.last()
    }
    pub(crate) fn last_move(&self) -> Option<usize> {
        self.moves.last().map(|m| m.get_piece())
    }
    pub(crate) fn do_move_ref(&mut self, m: &Move) -> Option<usize> {
        for motion in m.get_motions() {
            self.pieces[motion.get_piece()].goto(
                motion.get_rank(),
                motion.get_file(),
                self.moves.len() + 1,
            );
        }
        for capture in m.get_captures() {
            self.pieces[capture.get_piece()].capture();
        }
        Some(0)
    }
    pub(crate) fn do_move(&mut self, m: Move) -> Option<usize> {
        self.do_move_ref(&m);
        self.moves.push(m);
        Some(0)
    }
    pub(crate) fn undo_move(&mut self) -> Option<usize> {
        let m = self.moves.pop()?;
        for motion in m.get_motions() {
            self.pieces[motion.get_piece()].goto(
                motion.get_from_rank(),
                motion.get_from_file(),
                self.moves.len() + 1,
            );
            if self.pieces[motion.get_piece()].when_moved() >= self.moves.len() {
                self.pieces[motion.get_piece()].set_has_moved(0);
            }
        }
        for capture in m.get_captures() {
            self.pieces[capture.get_piece()].uncapture();
        }
        Some(0)
    }
    pub(crate) fn get_piece_at(&mut self, rank: &BigInt, file: &BigInt) -> Option<usize> {
        if *rank == 1.into() && !self.black_pawns.has_moved(file) {
            self.black_pawns.set_moved(file);
            self.place_piece(Piece::new(
                "pawn".to_string(),
                Color::Black,
                rank.clone(),
                file.clone(),
            ));
        }
        if *rank == 6.into() && !self.white_pawns.has_moved(file) {
            self.white_pawns.set_moved(file);
            self.place_piece(Piece::new(
                "pawn".to_string(),
                Color::White,
                rank.clone(),
                file.clone(),
            ));
        }
        for i in 0..self.pieces.len() {
            if self.pieces[i].get_rank() == rank
                && self.pieces[i].get_file() == file
                && !self.pieces[i].is_captured()
            {
                return Some(i);
            }
        }
        None
    }
    pub(crate) fn get_collision(
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
    pub(crate) fn move_legal_at_all(
        s: &mut Board,
        rules: &StandardChess,
        from_rank: &BigInt,
        from_file: &BigInt,
        to_rank: &BigInt,
        to_file: &BigInt,
    ) -> Option<Move> {
        if from_rank == to_rank && from_file == to_file {
            return None;
        }
        if let Some(p) = s.get_piece_at(from_rank, from_file) {
            let good_so_far = rules.can_move(s, p, to_rank, to_file);
            if let Some(other) = s.get_piece_at(to_rank, to_file) {
                if s.pieces[p].get_color() == s.pieces[other].get_color() {
                    return None;
                }
            }
            good_so_far
        } else {
            None //no piece there so not legal
        }
    }
    pub(crate) fn move_legal(
        s: &mut Board,
        rules: &StandardChess,
        from_rank: &BigInt,
        from_file: &BigInt,
        to_rank: &BigInt,
        to_file: &BigInt,
    ) -> Option<Move> {
        if from_rank == to_rank && from_file == to_file {
            return None;
        }
        if let Some(p) = s.get_piece_at(from_rank, from_file) {
            if let Some(x) = s.last_move() {
                if s.pieces[x].get_color() == s.pieces[p].get_color() {
                    return None;
                }
            } else {
                if s.pieces[p].get_color() != Color::White {
                    return None;
                }
            }
            let good_so_far =
                Self::move_legal_at_all(s, rules, from_rank, from_file, to_rank, to_file);
            good_so_far.and_then(|m| rules.would_be_in_check(s, m))
        } else {
            None //no piece there so not legal
        }
    }
    pub(crate) fn is_move_legal(
        s: &mut Board,
        rules: &StandardChess,
        from_rank: &BigInt,
        from_file: &BigInt,
        to_rank: &BigInt,
        to_file: &BigInt,
    ) -> bool {
        Self::move_legal(s, rules, from_rank, from_file, to_rank, to_file).is_some()
    }
}
