use std::sync::Arc;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};

use actix_web::{get, put, HttpResponse};
use piece::Piece;
use rusqlite::Connection;
use crate::piece_rules::StandardChess;

mod piece;
mod board;
mod piece_rules;
mod pawn_rank;
mod board_serializer;
mod error;
mod piece_serializer;

use board::Board;
use board_serializer::board_serialize;
use error::*;
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut b = Board::new();
    b.place_piece(Piece::new("pawn".to_string(), piece::Color::Black, 1.into(), 0.into()));
    let board = web::Data::new(Arc::new(Mutex::new(b)));
    HttpServer::new(move || App::new().service(fs::Files::new("/", "./static").index_file("index.html")).service(get).app_data(board.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}


#[get("/board")]
pub async fn get(
    board: web::Data<Arc<Mutex<Board>>>,
) -> Result<HttpResponse, Error> {
    let b = board.lock().unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(board_serialize(&b)))
}
