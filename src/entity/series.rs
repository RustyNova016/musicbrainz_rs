use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use crate::query::browse::impl_browse_includes;
use crate::query::relations::impl_relations_includes;
use serde::{Deserialize, Serialize};

use lucene_query_builder::QueryBuilder;

/// A series is a sequence of separate release groups, releases, recordings, works, artists or
/// events with a common theme.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Series {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    /// The series name is the official name of the series.
    pub name: String,
    #[serde(rename = "type")]
    /// The type primarily describes what type of entity the series contains. The possible values are:
    /// Release group series, Release series, Recording series, Work series (with further subtypes:
    /// Catalogue), Artist series (with further subtypes: Artist award), Event series (with further
    /// subtypes: Tour, Festival, Run, Residency)
    pub series_type: SeriesType,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: String,
    pub type_id: String,
    /// Relationships are a way to represent all the different ways in which entities are connected
    /// to each other and to URLs outside MusicBrainz.
    pub relations: Option<Vec<Relation>>,
    pub tags: Option<Vec<Tag>>,
    /// Aliases are alternate names for a series, which currently have two main functions: localised
    /// names and search hints.
    pub aliases: Option<Vec<Alias>>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
}

/// The type of a MusicBrainz series entity.
/// Note that this enum is `non_exhaustive`; The list of series types is subject to change and these
/// changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `series_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SeriesType {
    /// A series of release groups.
    #[serde(rename = "Release group series")]
    ReleaseGroupSeries,
    /// A series of releases.
    #[serde(rename = "Release series")]
    ReleaseSeries,
    /// A series of recordings.
    #[serde(rename = "Recording series")]
    RecordingSeries,
    /// A series of works.
    #[serde(rename = "Work series")]
    WorkSeries,
    /// A series of works which form a catalogue of classical compositions.
    Catalogue,
    /// A series of artists.
    #[serde(rename = "Artist series")]
    ArtistSeries,
    /// A series of artists honoured by the same award.
    #[serde(rename = "Artist award")]
    ArtistAward,
    /// A series of events.
    #[serde(rename = "Event series")]
    EventSeries,
    /// A series of related concerts by an artist in different locations.
    Tour,
    /// A recurring festival, usually happening annually in the same location.
    Festival,
    /// A series of performances of the same show at the same venue.
    Run,
    /// A series of related concerts by an artist in the same location.
    Residency,
    /// Any series_type that does not yet have a corresponding variant in this enum.
    /// If you ever see a `SeriesType::UnrecognizedSeriesType` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedSeriesType,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct SeriesSearchQuery {
    /// (part of) any alias attached to the series (diacritics are ignored)
    pub alias: String,
    /// (part of) the series' disambiguation comment
    pub comment: String,
    /// (part of) the series' name (diacritics are ignored)
    pub series: String,
    /// (part of) the series' name (with the specified diacritics)
    #[query_builder_field = "seriesaccent"]
    pub series_accent: String,
    /// the series' MBID
    pub sid: String,
    /// (part of) a tag attached to the series
    pub tag: String,
    /// the series' type
    #[query_builder_field = "type"]
    pub series_type: String,
}

impl_browse!(Series, (by_collection, BrowseBy::Collection));

impl_browse_includes!(
    Series,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_aliases, Include::Other("aliases"))
);

impl_includes!(
    Series,
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);

// Relationships includes
impl_relations_includes!(Series);
