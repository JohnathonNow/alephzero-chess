use std::{collections::HashSet, fmt};

use crate::piece::{Color, Piece};
use num_bigint::BigInt;
use num_traits::Signed;

pub struct PawnRank {
    /// PawnRank tracks the movement of the infinite number of pieces from a rank
    moved: HashSet<BigInt>,
}

impl PawnRank {
    pub fn new() -> Self {
        Self {
            moved: HashSet::new(),
        }
    }
    pub fn has_moved(&self, file: &BigInt) -> bool {
        self.moved.contains(file)
    }
    pub fn set_moved(&mut self, file: &BigInt) {
        self.moved.insert(file.clone());
    }
}

impl ToString for PawnRank {
    fn to_string(&self) -> String {
        self.moved.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
    }
}