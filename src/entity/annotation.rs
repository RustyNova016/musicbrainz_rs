use lucene_query_builder::QueryBuilder;
use serde::{Deserialize, Serialize};

/// Annotations are text fields, functioning like a miniature wiki, that can be added to any existing
/// artists, labels, recordings, releases, release groups and works.
/// Their purpose is to add information that usually doesn't fit into the strict structural data
/// schema of MusicBrainz / (be it due to technical limitations that may be addressed later, or
/// because the information in itself has to be free-text).
/// The content of an annotation can be edited by any MusicBrainz user. Like the rest of the database,
/// if something is incorrect or incomplete, you can fix it. All changes are recorded and if someone
/// deletes or defaces the annotation, you can easily restore a previous copy.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Annotation {
    /// the annotated entity's MBID
    pub entity: String,
    /// the annotated entity's name or title (diacritics are ignored)
    pub name: String,
    /// the annotation's content (includes wiki formatting)
    pub text: String,
    /// the annotated entity's entity type
    #[serde(rename = "type")]
    pub annotation_type: String,
    /// Relevance score of this entity (0–100). Present only in search querys.
    pub score: Option<u8>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct AnnotationSearchQuery {
    /// the annotated entity's MBID
    pub entity: String,
    /// the numeric ID of the annotation
    pub id: String,
    /// the annotated entity's name or title (diacritics are ignored)
    pub name: String,
    /// the annotation's content (includes wiki formatting)
    pub text: String,
    /// the annotated entity's entity type
    #[query_builder_field = "type"]
    pub annotation_type: String,
}
