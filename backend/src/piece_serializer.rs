use crate::piece::Piece;

pub fn piece_serialize(p: &Piece) -> String {
    let mut out = String::new();
    out += &format!("{{\"type\": \"{}_{}\", \"piece\": \"{}\", \"alive\": {}, \"y\": \"{}\", \"x\": \"{}\", \"color\": \"{}\", \"has_moved\": {}}}", p.get_color().to_string(), p.get_type(), p.get_type(), !p.is_captured(), p.get_rank(), p.get_file(),
p.get_color().to_string(), p.has_moved());
    out
}
