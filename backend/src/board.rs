use crate::pawn_rank::PawnRank;
use crate::piece::{Color, Piece};
use crate::piece_rules::{PieceRules, StandardChess};
use crate::piece_serializer::piece_serialize;
use num_bigint::BigInt;
use wasm_bindgen::prelude::*;
pub const STANDARD_BOARD_SIZE: i32 = 8;

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
    pub fn do_move(
        &mut self,
        rank: String,
        file: String,
        to_rank: String,
        to_file: String,
    ) -> Option<usize> {
        self.board.do_move(
            &rank.parse::<BigInt>().ok()?,
            &file.parse::<BigInt>().ok()?,
            &to_rank.parse::<BigInt>().ok()?,
            &to_file.parse::<BigInt>().ok()?,
        )
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
    pub fn get_legal_moves(
        &mut self,
        srank: String,
        sfile: String,
        swinx: String,
        swiny: String,
        szoom: String
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

pub struct Board {
    //pub(crate) turn: BigInt,
    pub white_can_castle: bool,
    pub black_can_castle: bool,
    pub(crate) pieces: Vec<Piece>,
    pub(crate) white_pawns: PawnRank,
    pub(crate) black_pawns: PawnRank,
}

impl Board {
    pub fn new() -> Self {
        Self {
            //turn: 0.into(),
            white_can_castle: true,
            black_can_castle: true,
            pieces: Vec::new(),
            white_pawns: PawnRank::new(),
            black_pawns: PawnRank::new(),
        }
    }
    pub(crate) fn place_piece(&mut self, piece: Piece) -> Option<usize> {
        let x = piece;
        self.pieces.push(x);
        Some(self.pieces.len() - 1)
    }
    pub(crate) fn do_move(
        &mut self,
        rank: &BigInt,
        file: &BigInt,
        to_rank: &BigInt,
        to_file: &BigInt,
    ) -> Option<usize> {
        let from = self.get_piece_at(rank, file)?;
        if let Some(to) = self.get_piece_at(to_rank, to_file) {
            self.pieces.get_mut(to).unwrap().capture();
        }
        let p = self.pieces.get_mut(from).unwrap();
        p.goto(to_rank, to_file);
        println!("{}", piece_serialize(p));
        None
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
    pub(crate) fn is_move_legal(
        s: &mut Board,
        rules: &StandardChess,
        from_rank: &BigInt,
        from_file: &BigInt,
        to_rank: &BigInt,
        to_file: &BigInt,
    ) -> bool {
        if from_rank == to_rank && from_file == to_file {
            return false;
        }
        if let Some(p) = s.get_piece_at(from_rank, from_file) {
            let good_so_far = rules.can_move(s, p, to_rank, to_file);
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
