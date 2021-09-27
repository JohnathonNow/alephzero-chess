use crate::board::Board;

pub fn board_serialize(b: &Board) -> String {
    let mut out = String::new();
    out += &format!("{{turn: {}, wc: {}, bc: {}}}", b.turn, b.white_can_castle, b.black_can_castle);
    out
}