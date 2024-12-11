use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::lifespan::LifeSpan;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use crate::query::browse::impl_browse_includes;
use crate::query::relations::impl_relations_includes;
use serde::{Deserialize, Serialize};

use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;

/// The type of a MusicBrainz event entity.
/// Note that this enum is `non_exhaustive`; The list of event types is subject to change and these
/// changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `event_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum EventType {
    /// An individual concert by a single artist or collaboration, often with supporting artists
    /// who perform before the main act.
    Concert,
    /// An event where a number of different acts perform across the course of the day. Larger
    /// festivals may be spread across multiple days.
    Festival,
    /// A performance of one or more plays, musicals, operas, ballets or other similar works for
    /// the stage in their staged form (as opposed to a <a
    /// href="https://en.wikipedia.org/wiki/Concert_performance">concert performance</a> without
    /// staging).
    #[serde(rename = "Stage performance")]
    StagePerformance,
    /// A party, reception or other event held specifically for the launch of a release.
    #[serde(rename = "Launch event")]
    LaunchEvent,
    /// A convention, expo or trade fair is an event which is not typically orientated around
    /// music performances, but can include them as side activities.
    #[serde(rename = "Convention/Expo")]
    ConventionExpo,
    /// A masterclass or clinic is an event where an artist meets with a small to medium-sized
    /// audience and instructs them individually and/or takes questions intended to improve the
    /// audience members' playing skills.
    #[serde(rename = "Masterclass/Clinic")]
    MasterclassClinic,
    /// Any event_type that does not yet have a corresponding variant in this enum.
    /// If you ever see a `EventType::UnrecognizedEventType` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedEventType,
}

/// An event refers to an organised event which people can attend, and is relevant to MusicBrainz.
/// Generally this means live performances, like concerts and festivals.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Event {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    /// The name is the official name of the event if it has one, or a descriptive name (like
    /// "Main Artist at Place") if not.
    pub name: String,

    /// The type describes what kind of event the event is. The possible values are: Concert,
    /// Festival, Launch event, Convention/Expo, Masterclass/Clinic
    #[serde(rename = "type")]
    pub event_type: Option<EventType>,
    /// The cancelled field describes whether or not the event took place.
    pub cancelled: Option<bool>,

    /// The time is the start time of the event.
    pub time: Option<String>,

    /// The setlist stores a list of songs performed, optionally including links to artists and works.
    /// See the setlist documentation for syntax and examples.
    // TODO: need some info on that value, current IT test returns ""
    pub setlist: Option<String>,
    // same here
    pub tags: Option<Vec<Tag>>,
    /// Relationships are a way to represent all the different ways in which entities are connected
    /// to each other and to URLs outside MusicBrainz.
    pub relations: Option<Vec<Relation>>,

    pub rating: Option<Rating>,
    /// Aliases are alternate names for an event, which currently have two main functions: localised
    /// names and search hints. Localised names are used to store the official names used in different
    /// languages and countries. These use the locale field to identify which language or country the
    /// name is for. Search hints are used to help both users and the server when searching and can
    /// be a number of things including alternate names, nicknames or even misspellings.
    pub aliases: Option<Vec<Alias>>,

    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,

    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,

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

    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities. They are visible in the pages for the entities, and
    /// also appear in the search results next to their names.
    pub disambiguation: Option<String>,

    pub type_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct EventSearchQuery {
    /// (part of) any alias attached to the artist (diacritics are ignored)
    pub alias: String,
    /// the MBID of an area related to the event
    pub aid: String,
    /// (part of) the name of an area related to the event
    pub area: String,
    /// the MBID of an artist related to the event
    pub arid: String,
    /// (part of) the name of an artist related to the event
    pub artist: String,
    /// the event's begin date (e.g. "1980-01-22")
    pub begin: Option<NaiveDate>,
    /// (part of) the artist's disambiguation comment
    pub comment: String,
    /// the event's end date (e.g. "1980-01-22")
    pub end: Option<NaiveDate>,
    /// a boolean flag (true/false) indicating whether or not the event has an end date set
    pub ended: bool,
    /// the MBID of the event
    pub eid: String,
    /// (part of) the event's name (diacritics are ignored)
    pub event: String,
    /// (part of) the event's name (with the specified diacritics)
    #[query_builder_field = "eventaccent"]
    pub event_accent: String,
    /// the MBID of a place related to the event
    pub pid: String,
    /// (part of) the name of a place related to the event
    pub place: String,
    /// (part of) a tag attached to the event
    pub tag: String,
    #[query_builder_field = "type"]
    /// the event's type
    pub event_type: String,
}

impl_includes!(
    Event,
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);

// Relationships includes
impl_relations_includes!(Event);

impl_browse! {
Event,
   (by_area, BrowseBy::Area),
   (by_collection, BrowseBy::Collection),
   (by_artist, BrowseBy::Artist),
   (by_place, BrowseBy::Place)
}

impl_browse_includes!(
    Event,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_aliases, Include::Other("aliases"))
);
