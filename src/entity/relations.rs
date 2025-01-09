use crate::date_format;
use crate::entity::area::Area;
use crate::entity::artist::Artist;
use crate::entity::event::Event;
use crate::entity::label::Label;
use crate::entity::place::Place;
use crate::entity::recording::Recording;
use crate::entity::release::Release;
use crate::entity::release_group::ReleaseGroup;
use crate::entity::series::Series;
use crate::entity::url::Url;
use crate::entity::work::Work;
use serde::{Deserialize, Serialize};

use chrono::NaiveDate;
use std::collections::HashMap;

/// Relationships are a way to represent all the different ways in which entities are connected to
/// each other and to URLs outside MusicBrainz.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Relation {
    #[serde(default)]
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub end: Option<NaiveDate>,
    /// Relationships can have attributes which modify the relationship. There is a
    /// [list of all attributes](https://musicbrainz.org/relationship-attributes), but the
    /// attributes which are available, and how they should be used, depends on the relationship
    /// type, so see the documentation for the relationship you want to use for more information.
    pub attributes: Option<Vec<String>>,
    #[serde(flatten)]
    pub content: RelationContent,
    pub attribute_values: Option<HashMap<String, String>>,
    pub attribute_ids: Option<HashMap<String, String>>,
    /// There are a huge number of different relationship types. The lists (organised per types of
    /// entities they connect) can be checked at the
    /// [relationship type table](https://musicbrainz.org/relationships).
    pub target_type: Option<String>,
    /// Credits allow indicating that, for example, songwriting was credited to an artist's legal
    /// name, and not his main (performance) name.
    pub target_credit: Option<String>,
    pub source_credit: Option<String>,
    pub ended: Option<bool>,
    pub type_id: String,
    #[serde(default)]
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    /// Some relationships have two date fields, a begin date and an end date, to store the period
    /// of time during which the relationship applied. The date can be the year, the year and the
    /// month or the full date. It is optional, so it can also be left blank. As with other attributes,
    /// see the documentation for the relationship types you are using.
    pub begin: Option<NaiveDate>,
    pub direction: String,
    #[serde(rename = "type")]
    pub relation_type: String,

    /// The order of the relationship
    pub ordering_key: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "snake_case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "snake_case"))]
pub enum RelationContent {
    // see https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    Artist(Box<Artist>),
    Area(Box<Area>),
    Event(Box<Event>),
    Label(Box<Label>),
    Place(Box<Place>),
    Recording(Box<Recording>),
    Release(Box<Release>),
    ReleaseGroup(Box<ReleaseGroup>),
    Series(Box<Series>),
    Url(Box<Url>),
    Work(Box<Work>),
}
