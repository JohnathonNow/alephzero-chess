use std::sync::Arc;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};

use actix_web::{get, put, HttpResponse};
use rusqlite::Connection;

mod piece;
mod board;
mod piece_rules;
mod pawn_rank;
mod board_serializer;
mod error;

use board::Board;
use board_serializer::board_serialize;
use error::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let board = web::Data::new(Arc::new(Mutex::new(Board::new())));
    HttpServer::new(move || App::new().service(get).app_data(board.clone()))
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
    Ok(HttpResponse::Ok().body(board_serialize(&b)))
}