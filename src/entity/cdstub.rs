use lucene_query_builder::QueryBuilder;
use serde::{Deserialize, Serialize};

/// A CD stub is an anonymously submitted track list that contains a disc ID, barcode, comment
/// field, and basic metadata like a release title and track names.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct CDStub {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    pub count: u32,
    /// the release title set on the CD stub
    pub title: String,
    /// the artist name set on the CD stub
    pub artist: String,
    /// the barcode set on the CD stub
    pub barcode: Option<String>,
    /// the disambiguation string set on the CD stub
    pub disambiguation: Option<String>,
    /// Relevance score of this entity (0–100). Present only in search querys.
    pub score: Option<u8>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct CDStubSearchQuery {
    /// the date the CD stub was added (e.g. "2020-01-22")
    pub added: String,
    /// (part of) the artist name set on the CD stub
    pub artist: String,
    /// the barcode set on the CD stub
    pub barcode: String,
    /// (part of) the comment set on the CD stub
    pub comment: String,
    /// the CD stub's Disc ID
    pub discid: String,
    /// (part of) the release title set on the CD stub
    pub title: String,
    /// the number of tracks on the CD stub
    pub tracks: u32,
}
