use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::area::Area;
use crate::entity::genre::Genre;
use crate::entity::lifespan::LifeSpan;
use crate::entity::rating::Rating;
use crate::entity::recording::Recording;
use crate::entity::relations::Relation;
use crate::entity::release::Release;
use crate::entity::release_group::ReleaseGroup;
use crate::entity::tag::Tag;
use crate::entity::work::Work;
use crate::entity::BrowseBy;
use crate::query::browse::impl_browse_includes;
use crate::query::relations::impl_relations_includes;
use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;
use serde::{Deserialize, Serialize};

/// An artist is generally a musician (or musician persona), group of musicians, or other music
/// professional (like a producer or engineer). Occasionally, it can also be a non-musical person
/// (like a photographer, an illustrator, or a poet whose writings are set to music), or even a
/// fictional character. For some other special cases, see special purpose artists.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
#[serde(default)]
pub struct Artist {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    /// The official name of an artist, be it a person or a band.
    pub name: String,
    /// The sort name is a variant of the artist name which would be used when sorting artists by
    /// name, such as in record shops or libraries. Among other things, sort names help to ensure
    /// that all the artists that start with "The" don't end up up under "T". The guidelines for
    /// sort names are the best place to check for more specific usage info.
    pub sort_name: String,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: String,

    /// The type is used to state whether an artist is a person, a group, or something else.
    #[serde(rename = "type")]
    pub artist_type: Option<ArtistType>,

    /// The gender is used to explicitly state whether a person or character identifies as male,
    /// female or neither. Groups do not have genders.
    pub gender: Option<Gender>,

    /// The artist area, as the name suggests, indicates the area with which an artist is primarily
    /// identified with. It is often, but not always, its birth/formation country.
    pub area: Option<Area>,

    ///The artist begin area, as the name suggests, indicates the area with which an artist started
    /// to perform.
    #[serde(rename = "begin_area")] // Forcing camel_case here since
    pub begin_area: Option<Area>, // all other field are deserialized in kebab-case

    pub relations: Option<Vec<Relation>>,
    /// release represents the unique release (i.e. issuing) of a product on a specific date with
    /// specific release information such as the country, label, barcode and packaging.
    pub releases: Option<Vec<Release>>,
    /// A work is a distinct intellectual or artistic creation, which can be expressed in the form of
    /// one or more audio recordings.
    pub works: Option<Vec<Work>>,
    pub release_groups: Option<Vec<ReleaseGroup>>,
    pub recordings: Option<Vec<Recording>>,

    /// Aliases are used to store alternate names or misspellings. For more information and examples,
    /// see the page about aliases.
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,

    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    pub rating: Option<Rating>,
    pub country: Option<String>,

    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any existing
    /// artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,

    /// The begin and end dates indicate when an artist started and finished its existence.
    /// Its exact meaning depends on the type of artist:
    ///
    ///  - For a person
    ///        Begin date represents date of birth, and end date represents date of death.
    ///
    ///    - For a group (or orchestra/choir)
    ///        Begin date represents the date when the group first formed: if a group dissolved and then
    ///        reunited, the date is still that of when they first formed. End date represents the date
    ///        when the group last dissolved: if a group dissolved and then reunited, the date is that
    ///        of when they last dissolved (if they are together, it should be blank!). For listing
    ///        other inactivity periods, just use the annotation and the "member of" relationships.
    ///
    ///    - For a character
    ///        Begin date represents the date (in real life) when the character concept was created.
    ///        The End date should not be set, since new media featuring a character can be created
    ///        at any time. In particular, the Begin and End date fields should not be used to hold
    ///        the fictional birth or death dates of a character.
    ///        (This information can be put in the annotation.)
    ///
    ///    - For others
    ///        There are no clear indications about how to use dates for artists of the type Other at
    ///        the moment.
    pub life_span: Option<LifeSpan>,
}

/// The type of a MusicBrainz artist entity.
/// Note that this enum is `non_exhaustive`; The list of artist types is subject to change and these
/// changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `artist_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ArtistType {
    /// This indicates a choir/chorus (an organized, usually large group of singers). Smaller
    /// vocal ensembles and groupings that do not generally call themselves choirs are better
    /// entered as “Group”.
    Choir,
    /// This indicates an orchestra (an organized, usually large group of instrumentalists).
    /// Smaller ensembles (such as trios and quartets) and groupings that do not generally call
    /// themselves orchestras are better entered as “Group”.
    Orchestra,
    /// This indicates an individual person, be it under its legal name (“John Lennon”), or a
    /// performance name (“Sting”).
    Person,
    /// A grouping of multiple musicians who perform together (in some cases, some or all of the
    /// members might differ in different performances or recordings).
    Group,
    /// This indicates an individual fictional character (whether a fictional person, animal or
    /// any other kind of character).
    Character,
    /// Anything which does not fit into the above categories.
    Other,
    /// Any artist_type that does not yet have a corresponding variant in this enum.
    /// If you ever see a `ArtistType::UnrecognizedArtistType` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedArtistType,
}

/// The gender is used to explicitly state whether a person or character identifies as male,
/// female or neither. Groups do not have genders.
/// The type of a MusicBrainz gender entity.
/// Note that this enum is `non_exhaustive`; The list of gender types is subject to change and these
/// changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `gender` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub enum Gender {
    Male,
    Female,
    #[default]
    Other,
    /// For cases where gender just doesn't apply at all (like companies entered as artists).
    #[serde(rename = "Not applicable")]
    NotApplicable,
    /// Any gender that does not yet have a corresponding variant in this enum.
    /// If you ever see a `Gender::UnrecognizedGender` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedGender,
}

#[derive(Debug, QueryBuilder, Default)]
pub struct ArtistSearchQuery {
    /// (part of) any alias attached to the artist (diacritics are ignored)
    pub alias: String,
    /// (part of) any primary alias attached to the artist (diacritics are ignored)
    #[query_builder_field = "primaryalias"]
    pub primary_alias: String,
    /// (part of) the name of the artist's main associated area
    pub area: String,
    /// the artist's MBID
    pub arid: String,
    /// (part of) the artist's name (diacritics are ignored)
    pub artist: String,
    /// (part of) the artist's name (with the specified diacritics)
    pub artist_accent: String,
    /// the artist's begin date (e.g. "1980-01-22")
    pub begin: Option<NaiveDate>,
    /// (part of) the name of the artist's begin area
    pub begin_area: String,
    /// (part of) the artist's disambiguation comment
    pub comment: String,
    /// the 2-letter code (ISO 3166-1 alpha-2) for the artist's main associated country
    pub country: String,
    /// the artist's end date (e.g. "1980-01-22")
    pub end: Option<NaiveDate>,
    /// (part of) the name of the artist's end area
    #[query_builder_field = "end_area"]
    pub end_area: String,
    /// a boolean flag (true/false) indicating whether or not the artist has ended (is dissolved/deceased)
    pub ended: bool,
    /// the artist's gender (“male”, “female”, “other” or “not applicable”)
    pub gender: Option<Gender>,
    /// an IPI code associated with the artist
    pub ipi: String,
    /// an ISNI code associated with the artist
    pub isni: String,
    /// (part of) the artist's sort name
    #[query_builder_field = "sortname"]
    pub sort_name: Option<String>,
    /// (part of) a tag attached to the artist
    pub tag: String,
    /// the artist's type (“person”, “group”, etc.)
    #[query_builder_field = "type"]
    pub artist_type: Option<ArtistType>,
}

impl_includes!(
    Artist,
    (with_recordings, Include::Subquery(Subquery::Recordings)),
    (with_releases, Include::Subquery(Subquery::Releases)),
    (with_medias, Include::Subquery(Subquery::Media)),
    (
        with_releases_and_discids,
        Include::Subquery(Subquery::ReleasesWithDiscIds)
    ),
    (
        with_release_groups,
        Include::Subquery(Subquery::ReleaseGroups)
    ),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_works, Include::Subquery(Subquery::Works)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_rating, Include::Subquery(Subquery::Rating)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);

// Relationships includes
impl_relations_includes!(Artist);

impl_browse! {
Artist,
   (by_area, BrowseBy::Area),
   (by_collection, BrowseBy::Collection),
   (by_recording, BrowseBy::Recording),
   (by_release, BrowseBy::Release),
   (by_release_group, BrowseBy::ReleaseGroup),
   (by_work, BrowseBy::Work)
}

impl_browse_includes!(
    Artist,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_aliases, Include::Other("aliases"))
);
