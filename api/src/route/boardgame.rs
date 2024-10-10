use std::sync::Arc;

use axum::{routing::post, Router};
use registry::AppModule;

use crate::handler::boardgame::create_board_game;

pub fn build_boardgame_routes() -> Router<Arc<AppModule>> {
    let router = Router::new().route("/", post(create_board_game));
    Router::new().nest("/boardgames", router)
}
