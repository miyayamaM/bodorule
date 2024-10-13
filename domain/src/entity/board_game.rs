use url::Url;
use uuid::Uuid;

pub struct BoardGame {
    pub id: Uuid,
    pub name: String,
    pub thumbnail_url: Option<Url>,
}
