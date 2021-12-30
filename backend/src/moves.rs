use num_bigint::BigInt;

pub struct Motion {
    piece: usize,
    to_rank: BigInt,
    to_file: BigInt,
}

impl Motion {
    pub(crate) fn get_piece(&self) -> usize {
        self.piece
    }
    pub(crate) fn get_rank(&self) -> &BigInt {
        &self.to_rank
    }
    pub(crate) fn get_file(&self) -> &BigInt {
        &self.to_file
    }
}

pub struct Captures {
    piece: usize,
}

impl Captures {
    pub(crate) fn get_piece(&self) -> usize {
        self.piece
    }
}

pub struct Move {
    piece: usize,
    motions: Vec<Motion>,
    captures: Vec<Captures>,
}
impl Move {
    pub fn new(piece: usize) -> Move {
        Self { piece, motions: Vec::new(), captures: Vec::new() }
    }
    pub fn standard(piece: usize, to_rank: &BigInt, to_file: &BigInt) -> Move {
        let mut move_ = Move::new(piece);
        move_.add_motion(piece, to_rank, to_file)
    }
    pub fn capture(piece: usize, to_rank: &BigInt, to_file: &BigInt, captured: usize) -> Move {
        Self::standard(piece, to_rank, to_file).add_capture(captured)
    }
    pub fn add_capture(mut self, piece: usize) -> Self {
        self.captures.push(Captures { piece });
        self
    }
    pub fn add_motion(mut self, piece: usize, to_rank: &BigInt, to_file: &BigInt) -> Self {
        self.motions.push(Motion { piece, to_rank: to_rank.clone(), to_file: to_file.clone() });
        self
    }
    pub fn get_piece(&self) -> usize {
        self.piece
    }
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        for motion in &self.motions {
            result.push_str(&format!("[{}, {}, {}]", motion.piece, motion.to_rank, motion.to_file));
        }
        for capture in &self.captures {
            result.push_str(&format!("[{}, {}]", capture.piece, "x"));
        }
        result
    }
    pub fn deserialize(s: &String) -> Move {
        Move::new(0)
    }

    pub(crate) fn get_motions(&self) -> &Vec<Motion> {
        &self.motions
    }

    pub(crate) fn get_captures(&self) -> &Vec<Captures> {
        &self.captures
    }
}
