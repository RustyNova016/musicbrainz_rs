use super::{Include, Relationship};
use crate::entity::tag::Tag;
use crate::query::relations::impl_relations_includes;
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

// Relationships includes
impl_relations_includes!(Url);

// impl_browse_includes!(
//     Recording,
//     // Common includes.
//     (with_annotation, Include::Other("annotation")),
//     (with_tags, Include::Other("tags")),
//     (with_user_tags, Include::Other("user-tags")),
//     (with_genres, Include::Other("genres")),
//     (with_user_genres, Include::Other("user-genres")),
//     (with_aliases, Include::Other("aliases"))
// );
