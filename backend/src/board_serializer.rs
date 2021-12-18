#[cfg(feature="server")]
use crate::board::Board;
#[cfg(feature="server")]
use crate::piece_serializer::piece_serialize;

#[cfg(feature="server")]
pub fn board_serialize(b: &Board) -> String {
    let mut out = String::new();
    let mut pieces = Vec::new();
    
    for piece in &b.pieces {
        pieces.push(piece_serialize(piece));
    }
    out += &format!("{{\"turn\": {}, \"wc\": {}, \"bc\": {}, \"pieces\": [{}], \"white_pawns\": [{}], \"black_pawns\": [{}]}}", 0 /*b.turn*/, b.white_can_castle, b.black_can_castle, pieces.join(","), b.white_pawns.to_string(), b.black_pawns.to_string());
    
    out
}