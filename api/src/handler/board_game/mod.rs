use domain::entity::board_game::BoardGame;
use serde::Serialize;
use uuid::Uuid;

pub mod create_board_game;
pub mod show_board_game;
pub mod show_board_games;

#[derive(Serialize)]
pub struct BoardGameResponse {
    id: Uuid,
    name: String,
    thumbnail_url: Option<String>,
}

impl From<BoardGame> for BoardGameResponse {
    fn from(board_game: BoardGame) -> Self {
        BoardGameResponse {
            id: board_game.id,
            name: board_game.name,
            thumbnail_url: board_game.thumbnail_url.map(|v| v.into()),
        }
    }
}
