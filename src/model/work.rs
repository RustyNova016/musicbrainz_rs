use crate::impl_includes;
use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::include::*;
use crate::model::rating::Rating;
use crate::model::tag::Tag;
use crate::BrowseBy;

/// In MusicBrainz terminology, a work is a distinct intellectual or artistic creation, which can be
/// expressed in the form of one or more audio recordings. While a work in MusicBrainz is usually
/// musical in nature, it is not necessarily so. For example, a work could be a novel, play,
/// poem or essay, later recorded as an oratory or audiobook.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Work {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    pub title: String,
    pub type_id: Option<String>,
    #[serde(rename = "type")]
    pub work_type: Option<String>,
    pub language: Option<String>,
    pub languages: Option<Vec<String>>,
    pub disambiguation: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Browse {
    Artist,
}

impl BrowseBy<Work> for Browse {
    fn as_str(&self) -> &str {
        match self {
            Browse::Artist => BROWSE_ARTIST_VALUE,
        }
    }
}

impl_includes!(
    Work,
    (with_artist_relations, Include::ArtistRelations),
    (with_tags, Include::Tags),
    (with_ratings, Include::Rating),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
