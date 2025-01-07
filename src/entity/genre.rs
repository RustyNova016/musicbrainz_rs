use serde::{Deserialize, Serialize};

/// Genres are currently supported in MusicBrainz as part of the tag system.
/// See [Genre](https://musicbrainz.org/doc/Genre) and
/// [supported genres](https://musicbrainz.org/genres) for more information.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Genre {
    pub id: Option<String>,
    pub count: Option<u32>,
    pub name: String,
    pub disambiguation: Option<String>,
}
