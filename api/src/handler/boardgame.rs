use axum::extract::{self, State};
use domain::entity::boardgame::Boardgame;
use domain::repository::boardgame::BoardgameRepository;
use serde::Deserialize;
use std::sync::Arc;
use url::Url;
use uuid::Uuid;

use registry::AppModule;
use shaku::HasComponent;

use crate::error::{AppError, ParseError};

pub async fn create_board_game(
    State(registry): State<Arc<AppModule>>,
    extract::Json(payload): extract::Json<CreateBoardGameRequest>,
) -> Result<(), AppError> {
    let boardgame_repository: &dyn BoardgameRepository = registry.resolve_ref();
    boardgame_repository.save(payload.try_into()?).await;
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct CreateBoardGameRequest {
    pub title: String,
    pub thumbnail_url: Option<String>,
}

impl TryFrom<CreateBoardGameRequest> for Boardgame {
    type Error = AppError;
    fn try_from(value: CreateBoardGameRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            title: value.title,
            thumbnail_url: value
                .thumbnail_url
                .map(|v| Url::parse(&v))
                .transpose()
                .map_err(ParseError::from)?,
        })
    }
}
