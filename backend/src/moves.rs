use num_bigint::BigInt;
use serde_json::Value;

#[derive(Clone)]
pub struct Motion {
    piece: usize,
    from_rank: BigInt,
    from_file: BigInt,
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
    pub(crate) fn get_from_rank(&self) -> &BigInt {
        &self.from_rank
    }
    pub(crate) fn get_from_file(&self) -> &BigInt {
        &self.from_file
    }
}

#[derive(Clone)]
pub struct Captures {
    piece: usize,
}

impl Captures {
    pub(crate) fn get_piece(&self) -> usize {
        self.piece
    }
}

#[derive(Clone)]
pub struct Move {
    piece: usize,
    motions: Vec<Motion>,
    captures: Vec<Captures>,
}
impl Move {
    pub fn new(piece: usize) -> Move {
        Self { piece, motions: Vec::new(), captures: Vec::new() }
    }
    pub fn standard(piece: usize, to_rank: &BigInt, to_file: &BigInt, from_rank: &BigInt, from_file: &BigInt) -> Move {
        let mut move_ = Move::new(piece);
        move_.add_motion(piece, to_rank, to_file, from_rank, from_file)
    }
    pub fn capture(piece: usize, to_rank: &BigInt, to_file: &BigInt, from_rank: &BigInt, from_file: &BigInt, captured: usize) -> Move {
        Self::standard(piece, to_rank, to_file, from_rank, from_file).add_capture(captured)
    }
    pub fn add_capture(mut self, piece: usize) -> Self {
        self.captures.push(Captures { piece });
        self
    }
    pub fn add_motion(mut self, piece: usize, to_rank: &BigInt, to_file: &BigInt, from_rank: &BigInt, from_file: &BigInt) -> Self {
        self.motions.push(Motion { piece, to_rank: to_rank.clone(), to_file: to_file.clone(), from_rank: from_rank.clone(), from_file: from_file.clone()  });
        self
    }
    pub fn get_piece(&self) -> usize {
        self.piece
    }
    pub fn serialize(&self) -> String {
        let mut result1 = Vec::new();
        let mut result2 = Vec::new();

        for motion in &self.motions {
            result1.push(format!("[{}, {}, {}]", motion.piece, motion.to_rank, motion.to_file));
        }
        for capture in &self.captures {
            result2.push(format!("[{}]", capture.piece));
        }
        format!("{{\"motions\": [{}], \"captures\": [{}], \"piece\": {}}}", result1.join(","), result2.join(","), self.piece)
    }
    pub fn deserialize(s: &String) -> Option<Move> {
        let v: Value = serde_json::from_str(s).ok()?;
        let mut m = Move::new(v["piece"].as_u64()? as usize);
        Some(m)
    }

    pub(crate) fn get_motions(&self) -> &Vec<Motion> {
        &self.motions
    }

    pub(crate) fn get_captures(&self) -> &Vec<Captures> {
        &self.captures
    }
}
