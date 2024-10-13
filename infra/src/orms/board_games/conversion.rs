use common::error::AppError;
use domain::entity::board_game::BoardGame;
use url::Url;

use super::Model;

impl TryFrom<Model> for BoardGame {
    type Error = AppError;
    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(BoardGame {
            id: value.id,
            name: value.name,
            thumbnail_url: value
                .thumbnail_url
                .map(|v| Url::parse(&v))
                .transpose()
                .map_err(|e| AppError::ConversionEntityError(e.to_string()))?,
        })
    }
}
