use crate::{board::Board, piece_rules::StandardChess, moves::Move};
use rand::prelude::*;


pub fn ai(b: &mut Board, rules: &StandardChess) -> Option<Move> {
    let mut rng = thread_rng();
    for i in 0..1000000 {
        let bigpx = rng.gen_range(-16..16);
        let bigpy = rng.gen_range(-16..16);
        let bigdx = rng.gen_range(-16..16);
        let bigdy = rng.gen_range(-16..16);
        if let Some(m) = Board::move_legal(b, &rules, &bigpx.into(), &bigpy.into(), &bigdx.into(), &bigdy.into()) {
            return Some(m);
        }
    }
    None
}
