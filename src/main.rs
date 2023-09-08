#[cfg(test)]
mod tests;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use rust_chess::Board;
use serde_json::to_string;

#[derive(serde::Deserialize)]
struct MoveJson {
    start: chess::Position,
    end: chess::Position,
    board: Board,
}

#[get("/new-board")]
async fn new_board() -> impl Responder {
    let board = Board::default();
    HttpResponse::Ok().body(to_string(&board).unwrap())
}

#[post("/move")]
async fn move_piece(body: String) -> actix_web::Result<impl Responder> {
    let mut move_json: MoveJson = serde_json::from_str(body.as_str())?;
    move_json.board.move_troop(move_json.start, move_json.end)?;

    Ok(HttpResponse::Ok().body(to_string(&move_json.board).unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(new_board))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
