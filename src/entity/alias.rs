use serde::{Deserialize, Serialize};

use crate::entity::date_string::DateString;

/// Aliases are used to store alternate names or misspellings. For more information and examples,
/// see the page about [aliases](https://musicbrainz.org/doc/Aliases).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
#[serde(default)]
pub struct Alias {
    pub name: String,
    pub sort_name: String,
    pub ended: Option<bool>,
    pub begin: Option<DateString>,
    pub end: Option<DateString>,
    #[serde(rename = "type")]
    pub alias_type: Option<String>,
    pub primary: Option<bool>,
    pub type_id: Option<String>,
}
