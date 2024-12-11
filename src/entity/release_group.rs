use super::{Include, Relationship, Subquery};
use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::release::Release;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use crate::query::browse::impl_browse_includes;
use crate::query::relations::impl_relations_includes;
use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;
use serde::{Deserialize, Serialize};

/// A release group, just as the name suggests, is used to group several different releases into a
/// single logical entity. Every release belongs to one, and only one release group.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
#[serde(default)]
pub struct ReleaseGroup {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    pub primary_type_id: Option<String>,

    /// The type describes what kind of releases the release group represents, for example album,
    /// single, soundtrack, compilation etc.
    /// See the Type subpage for a full list of release group types.
    pub primary_type: Option<ReleaseGroupPrimaryType>,

    pub secondary_type_ids: Vec<String>,
    pub secondary_types: Vec<ReleaseGroupSecondaryType>,

    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub first_release_date: Option<NaiveDate>,

    /// The title of a release group is usually very similar, if not the same, as the titles of the
    /// releases contained within it.
    pub title: String,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: String,
    /// Relationships are a way to represent all the different ways in which entities are connected
    /// to each other and to URLs outside MusicBrainz.
    pub relations: Option<Vec<Relation>>,
    /// Artist credits indicate who is the main credited artist (or artists) for releases, release
    /// groups, tracks and recordings, and how they are credited..
    pub artist_credit: Option<Vec<ArtistCredit>>,
    /// Releases present in this release group.
    pub releases: Option<Vec<Release>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    /// Aliases are alternate names for a release group.
    pub aliases: Option<Vec<Alias>>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
}

/// The primary type of a MusicBrainz release group.
/// Note that this enum is `non_exhaustive`; The list of release types is subject to change and
/// these changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `release_group_primary_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ReleaseGroupPrimaryType {
    Album,
    Single,
    #[serde(rename = "EP")]
    Ep,
    Broadcast,
    Other,
    /// Any release_group_primary_type that does not yet have a corresponding variant in this enum.
    /// If you ever see a `ReleaseGroupPrimaryType::UnrecognizedReleaseGroupPrimaryType` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedReleaseGroupPrimaryType,
}

/// The secondary type of a MusicBrainz release group entity.
/// Note that this enum is `non_exhaustive`; The list of release types is subject to change and
/// these changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `release_group_secondary_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ReleaseGroupSecondaryType {
    #[serde(rename = "Audio drama")]
    AudioDrama,
    Audiobook,
    Compilation,
    #[serde(rename = "DJ-mix")]
    DjMix,
    Demo,
    Interview,
    Live,
    #[serde(rename = "Mixtape/Street")]
    MixtapeStreet,
    Remix,
    Soundtrack,
    Spokenword,
    /// Any release_group_secondary_type that does not yet have a corresponding variant in this enum.
    /// If you ever see a `ReleaseGroupSecondaryType::UnrecognizedReleaseGroupSecondaryType` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedReleaseGroupSecondaryType,
}

#[derive(Debug, QueryBuilder, Default)]
pub struct ReleaseGroupSearchQuery {
    /// (part of) any alias attached to the release group (diacritics are ignored)
    pub alias: String,
    /// the MBID of any of the release group artists
    pub arid: String,
    /// (part of) the combined credited artist name for the release group, including join phrases (e.g. "Artist X feat.")
    pub artist: String,
    /// (part of) the name of any of the release group artists
    #[query_builder_field = "artistname"]
    pub artist_name: String,
    /// (part of) the release group's disambiguation comment
    pub comment: String,
    /// (part of) the credited name of any of the release group artists on this particular release group
    #[query_builder_field = "creditname"]
    pub credit_name: String,
    /// the release date of the earliest release in this release group (e.g. "1980-01-22")
    #[query_builder_field = "firstreleasedate"]
    pub first_release_date: String,
    /// the primary type of the release group
    #[query_builder_field = "primarytype"]
    pub primary_type: String,
    /// the MBID of any of the releases in the release group
    pub reid: String,
    /// (part of) the title of any of the releases in the release group
    pub release: String,
    /// (part of) the release group's title (diacritics are ignored)
    #[query_builder_field = "releasegroup"]
    pub release_group: String,
    /// (part of) the release group's title (with the specified diacritics)
    #[query_builder_field = "releasegroupaccent"]
    pub release_group_accent: String,
    /// the number of releases in the release group
    pub releases: String,
    /// the release group's MBID
    pub rgid: String,
    /// any of the secondary types of the release group
    #[query_builder_field = "secondarytype"]
    pub secondary_type: String,
    /// the status of any of the releases in the release group
    pub status: String,
    /// the status of any of the releases in the release group
    pub tag: String,
    /// legacy release group type field that predates the ability to set multiple types (see calculation code)
    #[query_builder_field = "type"]
    pub release_type: String,
}

impl_browse! {
ReleaseGroup,
   (by_artist, BrowseBy::Artist),
   (by_release, BrowseBy::Release),
   (by_collection, BrowseBy::Collection)
}

impl_browse_includes!(
    ReleaseGroup,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_artist_credits, Include::Other("artist-credits"))
);

impl_includes!(
    ReleaseGroup,
    (with_artists, Include::Subquery(Subquery::Artists)),
    (with_releases, Include::Subquery(Subquery::Releases)),
    (with_medias, Include::Subquery(Subquery::Media)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);

// Relationships includes
impl_relations_includes!(ReleaseGroup);
