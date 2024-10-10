use url::Url;
use uuid::Uuid;

pub struct Boardgame {
    id: Uuid,
    title: String,
    thumbnail_url: Url,
}
