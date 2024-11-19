use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::release::Release;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use crate::entity::{Include, Relationship, Subquery};
use crate::query::browse::impl_browse_includes;
use crate::query::relations::impl_relations_includes;
use serde::{Deserialize, Serialize};

use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;

/// A recording is an entity in MusicBrainz which can be linked to tracks on releases. Each track
/// must always be associated with a single recording, but a recording can be linked to any number
/// of tracks.
/// A recording represents distinct audio that has been used to produce at least one released track
/// through copying or mastering. A recording itself is never produced solely through copying or
/// mastering.
/// Generally, the audio represented by a recording corresponds to the audio at a stage in the
/// production process before any final mastering but after any editing or mixing.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Recording {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    /// The title of the recording.
    pub title: String,

    pub video: Option<bool>,
    /// The length of the recording. It's only entered manually for
    /// [standalone recordings](https://musicbrainz.org/doc/Standalone_Recording). For recordings
    /// that are being used on releases, the recording length is the median length of all tracks
    /// (that have a track length) associated with that recording. If there is an even number of
    /// track lengths, the smaller median candidate is used.
    pub length: Option<u32>, // TODO: CUSTOM Deserialized to make this a duration
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,
    /// The International Standard Recording Code assigned to the recording.
    pub isrcs: Option<Vec<String>>,
    pub relations: Option<Vec<Relation>>,
    pub releases: Option<Vec<Release>>,
    /// Artist credits indicate who is the main credited artist (or artists) for releases, release
    /// groups, tracks and recordings, and how they are credited.
    pub artist_credit: Option<Vec<ArtistCredit>>,
    /// Aliases are alternate names for a recording.
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
    /// The first release date of the recording.
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default)]
    pub first_release_date: Option<NaiveDate>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct RecordingSearchQuery {
    /// (part of) any alias attached to the recording (diacritics are ignored)
    pub alias: String,
    /// the MBID of any of the recording artists
    pub arid: String,
    /// (part of) the combined credited artist name for the recording, including join phrases
    /// (e.g. "Artist X feat.")
    pub artist: String,
    /// (part of) the name of any of the recording artists
    #[query_builder_field = "artistname"]
    pub artist_name: String,
    /// (part of) the recording's disambiguation comment
    pub comment: String,
    /// the 2-letter code (ISO 3166-1 alpha-2) for the country any release of this recording was released in
    pub country: String,
    /// (part of) the credited name of any of the recording artists on this particular recording
    #[query_builder_field = "creditname"]
    pub credit_name: String,
    /// the release date of any release including this recording (e.g. "1980-01-22")
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default)]
    pub date: Option<NaiveDate>,
    /// the recording duration in milliseconds
    #[query_builder_field = "dur"]
    pub duration: String,
    /// the release date of the earliest release including this recording (e.g. "1980-01-22")
    #[query_builder_field = "firstreleasedate"]
    pub first_release_date: String,
    /// the format of any medium including this recording (insensitive to case, spaces, and separators)
    pub format: String,
    /// any ISRC associated to the recording
    pub isrc: String,
    /// the free-text number of the track on any medium including this recording (e.g. "A4")
    pub number: String,
    /// the position inside its release of any medium including this recording (starts at 1)
    pub position: u32,
    /// the primary type of any release group including this recording
    #[query_builder_field = "primarytype"]
    /// the recording duration, quantized (duration in milliseconds / 2000)
    pub primary_type: String,
    /// (part of) the recording's name, or the name of a track connected to this recording (diacritics
    /// are ignored)
    #[query_builder_field = "qdur"]
    pub quantized_duration: String,
    /// (part of) the recording's name, or the name of a track connected to this recording (diacritics
    /// are ignored)
    pub recording: String,
    /// (part of) the recordings's name, or the name of a track connected to this recording (with
    /// the specified diacritics)
    #[query_builder_field = "recordingaccent"]
    pub recording_accent: String,
    /// the MBID of any release including this recording
    pub reid: String,
    /// (part of) the name of any release including this recording
    pub release: String,
    /// the MBID of any release group including this recording
    pub rgid: String,
    /// the recording's MBID
    pub rid: String,
    /// any of the secondary types of any release group including this recording
    #[query_builder_field = "secondarytype"]
    pub secondary_type: String,
    /// the status of any release including this recording
    pub status: String,
    /// (part of) a tag attached to the recording
    pub tag: String,
    /// the MBID of a track connected to this recording
    pub tid: String,
    /// the position of the track on any medium including this recording (starts at 1, pre-gaps at 0)
    pub tnum: u32,
    /// the number of tracks on any medium including this recording
    pub tracks: String,
    /// the number of tracks on any release (as a whole) including this recording
    #[query_builder_field = "tracksrelease"]
    pub tracks_release: String,
    /// legacy release group type field that predates the ability to set multiple types
    #[query_builder_field = "type"]
    pub recording_type: String,
    /// a boolean flag (true/false) indicating whether or not the recording is a video recording
    pub video: bool,
}

impl_browse! {
Recording,
   (by_release, BrowseBy::Release),
   (by_artist, BrowseBy::Artist),
   (by_work, BrowseBy::Work)
}

impl_includes!(
    Recording,
    (with_artists, Include::Subquery(Subquery::Artists)),
    (with_releases, Include::Subquery(Subquery::Releases)),
    (with_medias, Include::Subquery(Subquery::Media)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_isrcs, Include::Subquery(Subquery::ISRCs)),
    (
        with_work_level_relations,
        Include::Relationship(Relationship::WorkLevel)
    ),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);

impl_browse_includes!(
    Recording,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_artist_credits, Include::Other("artist-credits")),
    (with_isrcs, Include::Other("isrcs"))
);

// Relationships includes
impl_relations_includes!(Recording);
