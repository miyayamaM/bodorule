use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use registry::AppModule;

use crate::handler::board_game::create_board_game::create_board_game;
use crate::handler::board_game::show_board_game::show_board_game;
use crate::handler::board_game::show_board_games::show_board_games;

pub fn build_boardgame_routes() -> Router<Arc<AppModule>> {
    let router = Router::new()
        .route("/", post(create_board_game))
        .route("/:board_game_id", get(show_board_game))
        .route("/", get(show_board_games));
    Router::new().nest("/boardgames", router)
}
