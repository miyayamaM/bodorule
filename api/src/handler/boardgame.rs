use axum::{
    extract::{self, State},
    http::StatusCode,
};
use domain::entity::boardgame::Boardgame;
use domain::repository::boardgame::BoardgameRepository;
use serde::Deserialize;
use std::sync::Arc;
use url::Url;
use uuid::Uuid;

use registry::AppModule;
use shaku::HasComponent;

pub async fn create_board_game(
    State(registry): State<Arc<AppModule>>,
    extract::Json(payload): extract::Json<CreateBoardGameRequest>,
) -> StatusCode {
    let boardgame_repository: &dyn BoardgameRepository = registry.resolve_ref();
    println!("{:#?}", payload);
    boardgame_repository.save(payload.try_into().unwrap()).await;
    StatusCode::OK
}

#[derive(Deserialize, Debug)]
pub struct CreateBoardGameRequest {
    pub title: String,
    pub thumbnail_url: Option<String>,
}

impl TryFrom<CreateBoardGameRequest> for Boardgame {
    type Error = anyhow::Error;
    fn try_from(value: CreateBoardGameRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            title: value.title,
            thumbnail_url: value.thumbnail_url.and_then(|v| Url::parse(&v).ok()),
        })
    }
}
