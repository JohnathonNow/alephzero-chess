use std::fmt;

use num_bigint::BigInt;

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
#[derive(Clone, Copy)]

pub enum PieceType {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl PieceType {
    pub fn get_color(&self) -> Color {
        match self {
            PieceType::Pawn(c) => *c,
            PieceType::Knight(c) => *c,
            PieceType::Bishop(c) => *c,
            PieceType::Rook(c) => *c,
            PieceType::Queen(c) => *c,
            PieceType::King(c) => *c,
        }
    }
    pub fn flip(&self) -> Self {
        match self {
            PieceType::Pawn(c) => PieceType::Pawn(c.flip()),
            PieceType::Knight(c) => PieceType::Knight(c.flip()),
            PieceType::Bishop(c) => PieceType::Bishop(c.flip()),
            PieceType::Rook(c) => PieceType::Rook(c.flip()),
            PieceType::Queen(c) => PieceType::Queen(c.flip()),
            PieceType::King(c) => PieceType::King(c.flip()),
        }
    }
    pub fn symbol(&self) -> &str {
        match self {
            PieceType::Pawn(Color::White) => "P",
            PieceType::Pawn(Color::Black) => "p",
            PieceType::Knight(Color::White) => "N",
            PieceType::Knight(Color::Black) => "n",
            PieceType::Bishop(Color::White) => "B",
            PieceType::Bishop(Color::Black) => "b",
            PieceType::Rook(Color::White) => "R",
            PieceType::Rook(Color::Black) => "r",
            PieceType::Queen(Color::White) => "Q",
            PieceType::Queen(Color::Black) => "q",
            PieceType::King(Color::White) => "K",
            PieceType::King(Color::Black) => "k",
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Clone)]
pub struct Piece {
    pub piece: PieceType,
    pub rank: BigInt,
    pub file: BigInt,
    pub captured: bool,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(piece: PieceType, rank: BigInt, file: BigInt) -> Self {
        Self {
            piece,
            rank,
            file,
            captured: false,
            has_moved: false,
        }
    }
}
