use super::Include;
use crate::model::tag::Tag;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Url {
    pub id: String,
    pub resource: String,
    pub tags: Option<Vec<Tag>>,
}

impl_includes!(Url, (with_artist_relations, Include::ArtistRelations));
