use std::marker::PhantomData;

use crate::config::*;
use crate::entity::annotation::Annotation;
use crate::entity::area::Area;
use crate::entity::artist::Artist;
use crate::entity::cdstub::CDStub;
use crate::entity::coverart::Coverart;
use crate::entity::discid::Discid;
use crate::entity::event::Event;
use crate::entity::instrument::*;
use crate::entity::label::Label;
use crate::entity::place::Place;
use crate::entity::recording::Recording;
use crate::entity::release::Release;
use crate::entity::release_group::ReleaseGroup;
use crate::entity::series::Series;
use crate::entity::url::Url;
use crate::entity::work::Work;
use crate::Fetch;
use crate::Path;
use crate::{Browse, Search};
use crate::{CoverartQuery, FetchCoverart, FetchCoverartQuery};
use serde::Serialize;
#[cfg(not(feature = "legacy_serialize"))]
use serde::Serializer;

macro_rules! impl_includes {
    ($ty: ty, $(($args:ident, $inc: expr)),+) => {
        use crate::{FetchQuery, BrowseQuery, SearchQuery};
        impl FetchQuery<$ty> {
               $(pub fn $args(&mut self) -> &mut Self  {
                     self.0.include = self.0.include($inc).include.to_owned();
                   self
               })*
            }

        impl BrowseQuery<$ty> {
               $(pub fn $args(&mut self) -> &mut Self  {
                     self.inner.include = self.inner.include($inc).include.to_owned();
                   self
               })*
            }

        impl SearchQuery<$ty> {
               $(pub fn $args(&mut self) -> &mut Self  {
                     self.inner.include = self.inner.include($inc).include.to_owned();
                   self
               })*
            }
        }
}

macro_rules! impl_browse {
    ($ty: ty, $(($args:ident, $browse: expr)),+) => {
        impl BrowseQuery<$ty> {
               $(pub fn $args(&mut self, id: &str) -> &mut Self  {
                    use std::fmt::Write as _;
                    self.inner.path.push_str(crate::config::FMT_JSON);
                    let _ = write!(self.inner.path, "&{}={}", $browse.as_str(), id);
                    self
               })*
            }
        }
}

macro_rules! impl_fetchcoverart {
    ($($t: ty), +) => {
        $(impl<'a> FetchCoverart<'a> for $t {
            fn get_coverart(&self) -> FetchCoverartQuery<Self> {
                let mut coverart_query = FetchCoverartQuery(CoverartQuery {
                    path: format!("{}/{}", BASE_COVERART_URL, Self::path()),
                    phantom: PhantomData,
                    target: CoverartTarget {
                        img_type: None,
                        img_res: None,
                    },
                });
                coverart_query.id(&self.id);
                coverart_query
            }
        })+
    }
}

pub mod alias;
pub mod annotation;
pub mod api;
pub mod area;
pub mod artist;
pub mod artist_credit;
pub mod cdstub;
pub mod coverart;
pub mod discid;
pub mod event;
pub mod genre;
pub mod instrument;
pub mod label;
pub mod lifespan;
pub mod place;
pub mod rating;
pub mod recording;
pub mod relations;
pub mod release;
pub mod release_group;
pub mod search;
pub mod series;
pub mod tag;
pub mod url;
pub mod work;

impl Fetch<'_> for Artist {}
impl Fetch<'_> for Recording {}
impl Fetch<'_> for ReleaseGroup {}
impl Fetch<'_> for Release {}
impl Fetch<'_> for Work {}
impl Fetch<'_> for Label {}
impl Fetch<'_> for Area {}
impl Fetch<'_> for Event {}
impl Fetch<'_> for Instrument {}
impl Fetch<'_> for Place {}
impl Fetch<'_> for Series {}
impl Fetch<'_> for Url {}
impl Fetch<'_> for Discid {}

impl_fetchcoverart!(Release, ReleaseGroup);

impl Browse<'_> for Artist {}
impl Browse<'_> for Area {}
impl Browse<'_> for Recording {}
impl Browse<'_> for ReleaseGroup {}
impl Browse<'_> for Release {}
impl Browse<'_> for Label {}
impl Browse<'_> for Event {}
impl Browse<'_> for Place {}
impl Browse<'_> for Work {}
impl Browse<'_> for Instrument {}
impl Browse<'_> for Series {}

impl Search<'_> for Area {}
impl Search<'_> for Annotation {}
impl Search<'_> for Artist {}
impl Search<'_> for Event {}
impl Search<'_> for Instrument {}
impl Search<'_> for Label {}
impl Search<'_> for Recording {}
impl Search<'_> for Release {}
impl Search<'_> for ReleaseGroup {}
impl Search<'_> for Series {}
impl Search<'_> for Work {}
impl Search<'_> for CDStub {}

impl Path<'_> for Annotation {
    fn path() -> &'static str {
        "annotation"
    }
}

impl Path<'_> for Artist {
    fn path() -> &'static str {
        "artist"
    }
}

impl Path<'_> for Recording {
    fn path() -> &'static str {
        "recording"
    }
}

impl Path<'_> for ReleaseGroup {
    fn path() -> &'static str {
        "release-group"
    }
}

impl Path<'_> for Release {
    fn path() -> &'static str {
        "release"
    }
}

impl Path<'_> for Work {
    fn path() -> &'static str {
        "work"
    }
}

impl Path<'_> for Label {
    fn path() -> &'static str {
        "label"
    }
}

impl Path<'_> for Area {
    fn path() -> &'static str {
        "area"
    }
}

impl Path<'_> for Event {
    fn path() -> &'static str {
        "event"
    }
}

impl Path<'_> for Instrument {
    fn path() -> &'static str {
        "instrument"
    }
}

impl Path<'_> for Place {
    fn path() -> &'static str {
        "place"
    }
}

impl Path<'_> for Series {
    fn path() -> &'static str {
        "series"
    }
}

impl Path<'_> for Url {
    fn path() -> &'static str {
        "url"
    }
}

impl Path<'_> for CDStub {
    fn path() -> &'static str {
        "cdstub"
    }
}

impl Path<'_> for Discid {
    fn path() -> &'static str {
        "discid"
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(unused)]
pub(crate) enum Include {
    Subquery(Subquery),
    Relationship(Relationship),
}

impl Include {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Include::Subquery(i) => i.as_str(),
            Include::Relationship(i) => i.as_str(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(unused)]
pub(crate) enum Subquery {
    Urls,
    Areas,
    ArtistCredits,
    Labels,
    Events,
    Places,
    DiscIds,
    Releases,
    ReleasesWithDiscIds,
    ReleaseGroups,
    Recordings,
    Aliases,
    Works,
    Tags,
    Rating,
    Genres,
    Annotations,
    Artists,
    Series,
    Instruments,
    ISRCs,
    Media,
}

impl Subquery {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Subquery::Labels => "labels",
            Subquery::Recordings => "recordings",
            Subquery::Tags => "tags",
            Subquery::Rating => "ratings",
            Subquery::Aliases => "aliases",
            Subquery::Genres => "genres",
            Subquery::Annotations => "annotation",
            Subquery::Releases => "releases",
            Subquery::ReleaseGroups => "release-groups",
            Subquery::Works => "works",
            Subquery::Artists => "artists",
            Subquery::Places => "places",
            Subquery::Events => "events",
            Subquery::Urls => "urls",
            Subquery::Areas => "areas",
            Subquery::ArtistCredits => "artist-credits",
            Subquery::DiscIds => "discids",
            Subquery::ReleasesWithDiscIds => "releases+discids",
            Subquery::Instruments => "instruments",
            Subquery::Series => "series",
            Subquery::ISRCs => "isrcs",
            Subquery::Media => "media",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(unused)]
pub(crate) enum Relationship {
    Area,
    Artist,
    Event,
    Instrument,
    Label,
    Place,
    Recording,
    Release,
    ReleaseGroup,
    Series,
    Url,
    Work,
    RecordingLevel,
    ReleaseGroupLevel,
    WorkLevel,
}

impl Relationship {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            // Main entity relations
            Relationship::Area => "area-rels",
            Relationship::Artist => "artist-rels",
            Relationship::Event => "event-rels",
            Relationship::Instrument => "instrument-rels",
            Relationship::Label => "label-rels",
            Relationship::Place => "place-rels",
            Relationship::Recording => "recording-rels",
            Relationship::Release => "release-rels",
            Relationship::ReleaseGroup => "release-group-rels",
            Relationship::Series => "series-rels",
            Relationship::Url => "url-rels",
            Relationship::Work => "work-rels",

            // Special relations
            Relationship::RecordingLevel => "recording-level-rels",
            Relationship::ReleaseGroupLevel => "release-group-level-rels",
            Relationship::WorkLevel => "work-level-rels",
        }
    }
}

pub(crate) enum BrowseBy {
    Area,
    Artist,
    Recording,
    Release,
    ReleaseGroup,
    Work,
    Collection,
    Place,
    Label,
    Track,
    TrackArtist,
}

impl BrowseBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            BrowseBy::Artist => "artist",
            BrowseBy::Area => "area",
            BrowseBy::Collection => "collection",
            BrowseBy::Recording => "recording",
            BrowseBy::Release => "release",
            BrowseBy::ReleaseGroup => "release-group",
            BrowseBy::Work => "work",
            BrowseBy::Place => "place",
            BrowseBy::Label => "label",
            BrowseBy::Track => "track",
            BrowseBy::TrackArtist => "track_artist",
        }
    }
}

/// Browse query result are wrapped in this generic struct and paired with a custom
/// Deserialize implementation to avoid reimplementing a custom deserializer for every entity.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    derive(Serialize),
    serde(rename_all(deserialize = "kebab-case"))
)]
pub struct BrowseResult<T> {
    pub count: i32,
    pub offset: i32,
    pub entities: Vec<T>,
}

#[cfg(not(feature = "legacy_serialize"))]
impl<T> Serialize for BrowseResult<T>
where
    T: Browsable + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry(T::COUNT_FIELD, &self.count)?;
        map.serialize_entry(T::OFFSET_FIELD, &self.offset)?;
        map.serialize_entry(T::ENTITIES_FIELD, &self.entities)?;
        map.end()
    }
}

pub trait Browsable {
    const COUNT_FIELD: &'static str;
    const OFFSET_FIELD: &'static str;
    const ENTITIES_FIELD: &'static str;
}

impl Browsable for Artist {
    const COUNT_FIELD: &'static str = "artist-count";
    const OFFSET_FIELD: &'static str = "artist-offset";
    const ENTITIES_FIELD: &'static str = "artists";
}

impl Browsable for Event {
    const COUNT_FIELD: &'static str = "event-count";
    const OFFSET_FIELD: &'static str = "event-offset";
    const ENTITIES_FIELD: &'static str = "events";
}

impl Browsable for Label {
    const COUNT_FIELD: &'static str = "label-count";
    const OFFSET_FIELD: &'static str = "label-offset";
    const ENTITIES_FIELD: &'static str = "labels";
}

impl Browsable for Place {
    const COUNT_FIELD: &'static str = "place-count";
    const OFFSET_FIELD: &'static str = "place-offset";
    const ENTITIES_FIELD: &'static str = "places";
}

impl Browsable for Recording {
    const COUNT_FIELD: &'static str = "recording-count";
    const OFFSET_FIELD: &'static str = "recording-offset";
    const ENTITIES_FIELD: &'static str = "recordings";
}

impl Browsable for Release {
    const COUNT_FIELD: &'static str = "release-count";
    const OFFSET_FIELD: &'static str = "release-offset";
    const ENTITIES_FIELD: &'static str = "releases";
}

impl Browsable for ReleaseGroup {
    const COUNT_FIELD: &'static str = "release-group-count";
    const OFFSET_FIELD: &'static str = "release-group-offset";
    const ENTITIES_FIELD: &'static str = "release-groups";
}

impl Browsable for Series {
    const COUNT_FIELD: &'static str = "series-count";
    const OFFSET_FIELD: &'static str = "series-offset";
    const ENTITIES_FIELD: &'static str = "series";
}

impl Browsable for Work {
    const COUNT_FIELD: &'static str = "work-count";
    const OFFSET_FIELD: &'static str = "work-offset";
    const ENTITIES_FIELD: &'static str = "works";
}

impl Browsable for Area {
    const COUNT_FIELD: &'static str = "area-count";
    const OFFSET_FIELD: &'static str = "area-offset";
    const ENTITIES_FIELD: &'static str = "areas";
}

impl Browsable for Instrument {
    const COUNT_FIELD: &'static str = "instrument-count";
    const OFFSET_FIELD: &'static str = "instrument-offset";
    const ENTITIES_FIELD: &'static str = "instruments";
}

#[derive(Clone, Debug)]
pub struct CoverartTarget {
    pub img_type: Option<CoverartType>,
    pub img_res: Option<CoverartResolution>,
}

#[derive(Clone, Debug)]
pub enum CoverartResponse {
    Json(Coverart),
    Url(String),
}

#[derive(Clone, Debug)]
pub enum CoverartType {
    Front,
    Back,
}

impl CoverartType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CoverartType::Front => "front",
            CoverartType::Back => "back",
        }
    }
}

#[derive(Clone, Debug)]
pub enum CoverartResolution {
    Res250,
    Res500,
    Res1200,
}

impl CoverartResolution {
    pub fn as_str(&self) -> &'static str {
        match self {
            CoverartResolution::Res250 => "250",
            CoverartResolution::Res500 => "500",
            CoverartResolution::Res1200 => "1200",
        }
    }
}
