use num_bigint::BigInt;

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
        }
        .into()
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
    pub fn set_type(&mut self, piece: String) {
        self.piece = piece;
    }
    #[cfg(not(features = "server"))]
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
    #[cfg(not(features = "server"))]
    pub fn has_moved(&self) -> bool {
        self.has_moved
    }
    #[cfg(not(features = "server"))]
    pub fn set_has_moved(&mut self) {
        self.has_moved = true;
    }
    #[cfg(not(features = "server"))]
    pub(crate) fn capture(&mut self) {
        self.captured = true;
    }
    #[cfg(not(features = "server"))]
    pub(crate) fn goto(&mut self, rank: &BigInt, file: &BigInt) {
        self.rank = rank.clone();
        self.file = file.clone();
        self.has_moved = true;
    }
}