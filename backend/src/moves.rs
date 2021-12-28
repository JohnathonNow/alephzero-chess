pub struct Move {
    piece: usize, // index of piece that was moved
}
impl Move {

pub fn get_piece(&self) -> usize {
    self.piece
}

pub(crate) fn new(from: usize) -> Move {
    Self {
        piece: from,
    }
}
}