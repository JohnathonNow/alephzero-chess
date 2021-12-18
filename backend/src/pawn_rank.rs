use std::{collections::HashSet, cell::{RefCell}};

use num_bigint::BigInt;

pub struct PawnRank {
    /// PawnRank tracks the movement of the infinite number of pieces from a rank
    moved: RefCell<HashSet<BigInt>>,
}

impl PawnRank {
    pub fn new() -> Self {
        Self {
            moved: RefCell::new(HashSet::new()),
        }
    }
    pub fn has_moved(&self, file: &BigInt) -> bool {
        self.moved.borrow().contains(file)
    }
    pub fn set_moved(&self, file: &BigInt) {
        self.moved.borrow_mut().insert(file.clone());
    }
}

impl ToString for PawnRank {
    fn to_string(&self) -> String {
        self.moved.borrow().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
    }
}