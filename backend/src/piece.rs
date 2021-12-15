use std::{collections::HashMap, fmt};
use num_bigint::BigInt;

use crate::board::Board;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Black => "black",
            Color::White => "white",
        }.into()
    }
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
    piece: String,
    rank: BigInt,
    file: BigInt,
    captured: bool,
    has_moved: bool,
    color: Color,
}

impl Piece {
    pub fn new(piece: String, color: Color, rank: BigInt, file: BigInt) -> Self {
        Self {
            piece,
            color,
            rank,
            file,
            captured: false,
            has_moved: false,
        }
    }
    pub fn get_type(&self) -> &String {
        &self.piece
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
    pub fn is_captured(&self) -> bool {
        self.captured
    }
    pub fn has_moved(&self) -> bool {
        self.has_moved
    }
}
