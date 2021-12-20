use num_bigint::BigInt;
use serde_json::Value;

use crate::piece::{Color, Piece};

pub fn piece_serialize(p: &Piece) -> String {
    let mut out = String::new();
    out += &format!("{{\"type\": \"{}_{}\", \"piece\": \"{}\", \"alive\": {}, \"y\": \"{}\", \"x\": \"{}\", \"color\": \"{}\", \"has_moved\": {}}}", p.get_color().to_string(), p.get_type(), p.get_type(), !p.is_captured(), p.get_rank(), p.get_file(),
p.get_color().to_string(), p.has_moved());
    out
}

pub fn piece_deserialize(s: &String) -> Option<Piece> {
    let v: Value = serde_json::from_str(s).ok()?;
    let mut p = Piece::new(
        v["piece"].as_str()?.to_string(),
        if v["color"].as_str()?.to_string() == "white" {
            Color::White
        } else {
            Color::Black
        },
        v["y"].as_str()?.to_string().parse::<BigInt>().ok()?,
        v["x"].as_str()?.to_string().parse::<BigInt>().ok()?,
    );
    if !v["alive"].as_bool()? {
        p.capture();
    }

    //TODO do this better
    Some(p)
}
