use serde::Deserialize;
use serde::Serialize;

use crate::api::impl_browse_includes::impl_browse_includes;
use crate::api::impl_relations_includes::impl_relations_includes;
use crate::entity::BrowseBy;
use crate::entity::Include;
use crate::entity::Subquery;
use crate::entity::alias::Alias;
use crate::entity::area::Area;
use crate::entity::genre::Genre;
use crate::entity::lifespan::LifeSpan;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;

pub use crate::entity::coordinates::Coordinate;
pub use crate::entity::coordinates::Coordinates;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Place {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    /// The place name is the official name of a place.
    pub name: String,
    /// The type categorises the place based on its primary function. The possible values are:
    /// Studio, Venue, Stadium, Indoor arena, Religious building, Educational institution,
    /// Pressing plant, Other.
    #[serde(rename = "type")]
    pub place_type: Option<PlaceType>,
    pub type_id: Option<String>,
    pub life_span: Option<LifeSpan>,
    /// The latitude and longitude describe the location of the place using geographic coordinates.
    pub coordinates: Option<Coordinates>,
    pub relations: Option<Vec<Relation>>,
    /// The area links to the area, such as the city, in which the place is located.
    pub area: Option<Area>,
    /// The address describes the location of the place using the standard addressing format for
    /// the country it is located in.
    pub address: Option<String>,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,
    /// Aliases are alternate names for a place, which currently have two main functions:
    /// localised names and search hints.
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
    /// Relevance score of this entity (0–100). Present only in search querys.
    pub score: Option<u8>,
}

/// The type of a MusicBrainz place entity.
/// Note that this enum is `non_exhaustive`; The list of place types is subject to change and these
/// changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `place_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum PlaceType {
    /// A place designed for non-live production of music, typically a recording studio.
    Studio,
    /// A place that has live artistic performances as one of its primary functions, such as a
    /// concert hall.
    Venue,
    /// A place whose main purpose is to host outdoor sport events, typically consisting of a pitch
    /// surrounded by a structure for spectators with no roof, or a roof which can be retracted.
    Stadium,
    /// A place consisting of a large enclosed area with a central event space surrounded by tiered
    /// seating for spectators, which can be used for indoor sports, concerts and other
    /// entertainment events.
    #[serde(rename = "Indoor arena")]
    IndoorArena,
    /// A school, university or other similar educational institution (especially, but not only, one
    /// where music is taught)
    #[serde(rename = "Educational institution")]
    EducationalInstitution,
    /// A place that has worship or religious studies as its main function. Religious buildings
    /// often host concerts and serve as recording locations, especially for classical music.
    #[serde(rename = "Religious building")]
    ReligiousBuilding,
    /// A place (generally a factory) at which physical media are manufactured.
    #[serde(rename = "Pressing plant")]
    PressingPlant,
    /// Anything which does not fit into the above categories.
    Other,
    /// Any place_type that does not yet have a corresponding variant in this enum.
    /// If you ever see a `PlaceType::UnrecognizedPlaceType` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedPlaceType,
}

impl_browse! {
Place,
   (by_area, BrowseBy::Area),
   (by_collection, BrowseBy::Collection)
}

impl_browse_includes!(
    Place,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_aliases, Include::Other("aliases"))
);

impl_includes!(
    Place,
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);

// Relationships includes
impl_relations_includes!(Place);
