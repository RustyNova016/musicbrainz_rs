use crate::entity::annotation::Annotation;
use crate::entity::area::Area;
use crate::entity::artist::Artist;
use crate::entity::cdstub::CDStub;
use crate::entity::event::Event;
use crate::entity::instrument::Instrument;
use crate::entity::label::Label;
use crate::entity::place::Place;
use crate::entity::recording::Recording;
use crate::entity::release::Release;
use crate::entity::release_group::ReleaseGroup;
use crate::entity::series::Series;
use crate::entity::tag::Tag;
use crate::entity::url::Url;
use crate::entity::work::Work;
use chrono::NaiveDateTime;
use serde::Serialize;
#[cfg(not(feature = "legacy_serialize"))]
use serde::Serializer;

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    derive(Serialize),
    serde(rename_all(deserialize = "kebab-case"))
)]
pub struct SearchResult<T> {
    pub created: NaiveDateTime,
    pub count: i32,
    pub offset: i32,
    pub entities: Vec<T>,
}

#[cfg(not(feature = "legacy_serialize"))]
impl<T> Serialize for SearchResult<T>
where
    T: Searchable + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap as _;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry(T::CREATED_FIELD, &self.created)?;
        map.serialize_entry(T::COUNT_FIELD, &self.count)?;
        map.serialize_entry(T::OFFSET_FIELD, &self.offset)?;
        map.serialize_entry(T::ENTITIES_FIELD, &self.entities)?;
        map.end()
    }
}

pub trait Searchable {
    const CREATED_FIELD: &'static str;
    const COUNT_FIELD: &'static str;
    const OFFSET_FIELD: &'static str;
    const ENTITIES_FIELD: &'static str;
}

impl Searchable for Annotation {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "annotations";
}

impl Searchable for Area {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "areas";
}

impl Searchable for Artist {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "artists";
}

impl Searchable for Event {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "events";
}

impl Searchable for Instrument {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "instruments";
}

impl Searchable for Label {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "labels";
}

impl Searchable for Place {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "places";
}

impl Searchable for Recording {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "recordings";
}

impl Searchable for Release {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "releases";
}

impl Searchable for ReleaseGroup {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "release-groups";
}

impl Searchable for Series {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "series";
}

impl Searchable for Tag {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "tags";
}

impl Searchable for Url {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "urls";
}

impl Searchable for Work {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "works";
}

impl Searchable for CDStub {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "cdstubs";
}
