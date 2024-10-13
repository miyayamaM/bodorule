use domain::entity::boardgame::Boardgame;
use serde::Serialize;
use uuid::Uuid;

pub mod create_board_game;
pub mod show_board_game;

#[derive(Serialize)]
pub struct BoardGameResponse {
    id: Uuid,
    name: String,
    thumbnail_url: Option<String>,
}

impl From<Boardgame> for BoardGameResponse {
    fn from(boardgame: Boardgame) -> Self {
        BoardGameResponse {
            id: boardgame.id,
            name: boardgame.name,
            thumbnail_url: boardgame.thumbnail_url.map(|v| v.into()),
        }
    }
}
