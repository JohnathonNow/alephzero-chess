use num_bigint::BigInt;
use serde_json::Value;

use crate::{board::Board, piece::{Color, Piece}};
//#[cfg(feature = "server")]
//use crate::board::Board;
use crate::piece_serializer::piece_deserialize;
use crate::piece_serializer::piece_serialize;

pub(crate) fn board_serialize(b: &Board) -> String {
    let mut out = String::new();
    let mut pieces = Vec::new();

    for piece in &b.pieces {
        pieces.push(piece_serialize(piece));
    }
    out += &format!("{{\"turn\": \"{}\", \"pieces\": [{}], \"white_pawns\": [{}], \"black_pawns\": [{}]}}", b.turn, pieces.join(","), b.white_pawns.to_string(), b.black_pawns.to_string());

    out
}

pub(crate) fn board_deserialize(b: &mut Board, s: &String) -> Option<i32> {
    b.white_pawns.clear();
    b.black_pawns.clear();
    b.pieces.clear();
    let v: Value = serde_json::from_str(s).ok()?;
    b.turn = v["turn"].as_str()?.parse::<BigInt>().ok()?;
    for p in v["pieces"].as_array()? {
        b.place_piece(piece_deserialize(&p.to_string())?);
    }
    for p in v["white_pawns"].as_array()? {
        b.white_pawns.set_moved(&p.to_string().parse::<BigInt>().ok()?);
    }
    for p in v["black_pawns"].as_array()? {
        b.black_pawns.set_moved(&p.to_string().parse::<BigInt>().ok()?);
    }
    Some(0)
}

#[test]
fn both_ways() {
    let mut b = Board::new();
    b.place_piece(Piece::new("pawn".to_string(), Color::White, 1.into(), 1.into()));
    b.place_piece(Piece::new("pawn".to_string(), Color::White, 2.into(), 1.into()));
    let s = board_serialize(&b);
    board_deserialize(&mut b, &s);
    assert_eq!(b.get_piece_at(&1.into(), &1.into()).unwrap(), 0);
    assert_eq!(b.pieces[0].get_type(), "pawn");
    assert_eq!(b.get_piece_at(&2.into(), &1.into()).unwrap(), 1);

}