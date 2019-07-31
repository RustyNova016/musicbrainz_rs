use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::rating::Rating;
use crate::model::release::Release;
use crate::model::tag::Tag;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Label {
    pub id: String,
    pub type_id: Option<String>,
    #[serde(rename = "type")]
    pub label_type: Option<String>,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub country: Option<String>,
    pub label_code: Option<u32>,
    pub releases: Option<Vec<Release>>,
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub genres: Option<Vec<Genre>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct LabelInfo {
    pub catalog_number: String,
    pub label: Label,
}

#[derive(Debug, PartialEq)]
pub enum Include {
    Releases,
    Aliases,
    Tags,
    Rating,
    Genres,
}

impl IncludeInto<Label> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::Releases => "releases",
            Include::Aliases => "aliases",
            Include::Tags => "tags",
            Include::Rating => "ratings",
            Include::Genres => "genres",
        }
    }
}
