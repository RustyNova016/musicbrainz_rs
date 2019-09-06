use crate::model::alias::Alias;
use crate::model::area::Area;
use crate::model::genre::Genre;
use crate::model::include_const::*;
use crate::model::lifespan::LifeSpan;
use crate::model::tag::Tag;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Place {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub place_type: String,
    pub type_id: String,
    pub life_span: LifeSpan,
    pub coordinates: Coordinates,
    pub area: Area,
    pub address: String,
    pub disambiguation: String,
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Include {
    ArtistRelations,
    Aliases,
    Tags,
    Genres,
    Annotation,
}

impl IncludeInto<Place> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => INC_ARTIST_REL_VALUE,
            Include::Aliases => INC_ALIASES_VALUE,
            Include::Tags => INC_TAGS_VALUE,
            Include::Genres => INC_GENRES_VALUE,
            Include::Annotation => INC_ANNOTATION_VALUE,
        }
    }
}
