use common::error::AppError;
use domain::entity::boardgame::Boardgame;
use url::Url;

use super::Model;

impl TryFrom<Model> for Boardgame {
    type Error = AppError;
    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(Boardgame {
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
