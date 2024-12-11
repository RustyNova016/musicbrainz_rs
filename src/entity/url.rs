use super::{Include, Relationship};
use crate::entity::tag::Tag;
use serde::{Deserialize, Serialize};

/// A URL in MusicBrainz is a specific entity representing a regular internet Uniform Resource Locator.
/// A MusicBrainz URL entity can be edited to change the underlying internet URL it points to; and can
/// be linked to Areas, Artists, Events, Instruments, Labels, Places, Recordings, Releases, Release
/// Groups, and Series.
///
/// Take a look at the [relationship table](https://musicbrainz.org/relationships) on the MusicBrainz
/// server to see all types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Url {
    pub id: String,
    pub resource: String,
    pub tags: Option<Vec<Tag>>,
}

impl_includes!(
    Url,
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url))
);
