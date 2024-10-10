use url::Url;
use uuid::Uuid;

pub struct Boardgame {
    id: Uuid,
    pub title: String,
    pub thumbnail_url: Option<Url>,
}
