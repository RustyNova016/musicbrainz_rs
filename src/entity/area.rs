use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::lifespan::LifeSpan;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use crate::query::browse::impl_browse_includes;
use crate::query::relations::impl_relations_includes;

use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;
use serde::{Deserialize, Serialize};

/// Areas are historical and existing geographic regions. Areas include countries, sub-divisions,
/// counties, municipalities, cities, districts and islands.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
#[serde(default)]
pub struct Area {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    /// The type of area. Possible values are: Country, Subdivision, County, Municipality, City,
    /// District, Island.
    #[serde(rename = "type")]
    pub area_type: Option<AreaType>,
    /// The name of the area.
    pub name: String,
    pub relations: Option<Vec<Relation>>,
    /// The ISO 3166 codes are the codes assigned by ISO to countries and subdivisions.
    pub iso_3166_1_codes: Option<Vec<String>>,
    /// The aliases are used to store alternate names or misspellings.
    pub aliases: Option<Vec<Alias>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: String,
    pub type_id: Option<String>,
    pub sort_name: String,
    pub life_span: Option<LifeSpan>,
    pub tags: Option<Vec<Tag>>,
    pub genres: Option<Vec<Genre>>,
}

/// The type of a MusicBrainz area entity.
/// Note that this enum is `non_exhaustive`; The list of area types is subject to change and these
/// changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `area_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AreaType {
    /// Country is used for areas included (or previously included) in ISO 3166-1, e.g. United
    /// States.
    Country,
    /// Subdivision is used for the main administrative divisions of a country, e.g. California,
    /// Ontario, Okinawa. These are considered when displaying the parent areas for a given
    /// area.
    Subdivision,
    /// City is used for settlements of any size, including towns and villages.
    City,
    /// Municipality is used for small administrative divisions which, for urban municipalities,
    /// often contain a single city and a few surrounding villages. Rural municipalities
    /// typically group several villages together.
    Municipality,
    /// District is used for a division of a large city, e.g. Queens.
    District,
    /// Island is used for islands and atolls which don't form subdivisions of their own, e.g.
    /// Skye. These are not considered when displaying the parent areas for a given area.
    Island,
    /// County is used for smaller administrative divisions of a country which are not the main
    /// administrative divisions but are also not municipalities, e.g. counties in the USA.
    /// These are not considered when displaying the parent areas for a given area.
    County,
    /// Used for any military bases that are large enough to be considered an area, not just a
    /// place.
    #[serde(rename = "Military base")]
    MilitaryBase,
    /// Used for semi-autonomous territories governed by indigenous peoples, such as Indian
    /// reserves/reservations in North America and indigenous territories in Central and South
    /// America.
    #[serde(rename = "Indigenous territory / reserve")]
    IndigenousTerritoryReserve,
    /// Any area_type that does not yet have a corresponding variant in this enum.
    /// If you ever see a `AreaType::UnrecognizedAreaType` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedAreaType,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct AreaSearchQuery {
    /// the area's MBID
    aid: String,
    /// (part of) any alias attached to the artist (diacritics are ignored)
    alias: Alias,
    /// (part of) the area's name (diacritics are ignored)
    area: String,
    /// (part of) the area's name (with the specified diacritics)
    #[query_builder_field = "areaaccent"]
    area_accent: String,
    /// the area's begin date (e.g. "1980-01-22")
    begin: Option<NaiveDate>,
    /// (part of) the area's disambiguation comment
    comment: String,
    /// the area's end date (e.g. "1980-01-22")
    end: Option<NaiveDate>,
    /// a boolean flag (true/false) indicating whether or not the area has ended (is no longer current)
    ended: bool,
    /// an ISO 3166-1, 3166-2 or 3166-3 code attached to the area
    iso: String,
    /// an ISO 3166-1 code attached to the area
    iso1: String,
    /// an ISO 3166-2 code attached to the area
    iso2: String,
    /// an ISO 3166-3 code attached to the area
    iso3: String,
    /// equivalent to name (areas no longer have separate sort names)
    #[query_builder_field = "sortname"]
    sort_name: String,
    /// (part of) a tag attached to the area
    tag: String,
    /// the area's type
    #[query_builder_field = "type"]
    area_type: String,
}

// Normal includes
impl_includes!(
    Area,
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);

// Relationships includes
impl_relations_includes!(Area);

impl_browse!(Area, (by_collection, BrowseBy::Collection));

impl_browse_includes!(
    Area,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_aliases, Include::Other("aliases"))
);
