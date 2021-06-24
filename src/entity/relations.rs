use crate::date_format;
use crate::entity::area::Area;
use crate::entity::artist::Artist;
use crate::entity::event::Event;
use crate::entity::url::Url;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Relation {
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub end: Option<NaiveDate>,
    pub attributes: Vec<String>,
    #[serde(flatten)]
    pub content: RelationContent,
    pub attribute_values: HashMap<String, String>,
    pub attribute_ids: HashMap<String, String>,
    pub target_credit: String,
    pub source_credit: String,
    pub ended: bool,
    pub type_id: String,
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub begin: Option<NaiveDate>,
    pub direction: String,
    #[serde(rename = "type")]
    pub relation_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum RelationContent {
    // see https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    Artist(Box<Artist>),
    Area(Box<Area>),
    Event(Box<Event>),
    Url(Box<Url>),
}
