use crate::date_format;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Alias {
    pub name: String,
    pub sort_name: String,
    pub ended: bool,
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub begin: Option<NaiveDate>,
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub end: Option<NaiveDate>,
    #[serde(rename = "type")]
    pub aliase_type: Option<String>,
    pub primary: Option<bool>,
    pub type_id: Option<String>,
}
