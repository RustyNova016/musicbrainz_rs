use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::release::Release;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use crate::entity::{Include, Relationship, Subquery};

/// A recording is an entity in MusicBrainz which can be linked to tracks on releases. Each track
/// must always be associated with a single recording, but a recording can be linked to any number
/// of tracks.
/// A recording represents distinct audio that has been used to produce at least one released track
/// through copying or mastering. A recording itself is never produced solely through copying or
/// mastering.
/// Generally, the audio represented by a recording corresponds to the audio at a stage in the
/// production process before any final mastering but after any editing or mixing.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
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

    /// See Disambiguation Comment.
    pub disambiguation: Option<String>,

    pub isrcs: Option<Vec<String>>,
    pub relations: Option<Vec<Relation>>,
    pub releases: Option<Vec<Release>>,
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
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
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_isrcs, Include::Subquery(Subquery::ISRCs)),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
