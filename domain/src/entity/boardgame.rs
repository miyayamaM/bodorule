use url::Url;
use uuid::Uuid;

pub struct Boardgame {
    pub id: Uuid,
    pub name: String,
    pub thumbnail_url: Option<Url>,
}
