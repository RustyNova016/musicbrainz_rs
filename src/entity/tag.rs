use core::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::client::MusicBrainzClient;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Tag {
    pub name: String,
    pub count: Option<i32>,
    pub score: Option<i32>,
}

// === Tag Submission ===

/// What type of vote to submit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagVote {
    /// Upvote the tag
    Upvote,

    /// Downvote the tag
    Downvote,

    /// Remove the current vote
    Withdraw,
}

impl Display for TagVote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagVote::Upvote => write!(f, "upvote"),
            TagVote::Downvote => write!(f, "downvote"),
            TagVote::Withdraw => write!(f, "withdraw"),
        }
    }
}

/// Tag data to submit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserTag {
    name: String,
    vote: TagVote,
}

impl UserTag {
    pub fn new(name: String, vote: TagVote) -> Self {
        Self { name, vote }
    }

    pub fn to_body_xml(&self) -> String {
        format!(
            r#"<user-tag vote="{}"><name>{}</name></user-tag>"#,
            self.vote, self.name,
        )
    }
}

/// The type of entity to tag
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaggableEntity {
    Artist,
    Event,
    Recording,
    Release,
    ReleaseGroup,
    Series,
    Work,
    Area,
    Instrument,
    Label,
    Place,
}

impl Display for TaggableEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaggableEntity::Artist => write!(f, "artist"),
            TaggableEntity::Event => write!(f, "event"),
            TaggableEntity::Recording => write!(f, "recording"),
            TaggableEntity::Release => write!(f, "release"),
            TaggableEntity::ReleaseGroup => write!(f, "release-group"),
            TaggableEntity::Series => write!(f, "series"),
            TaggableEntity::Work => write!(f, "work"),
            TaggableEntity::Area => write!(f, "area"),
            TaggableEntity::Instrument => write!(f, "instrument"),
            TaggableEntity::Label => write!(f, "label"),
            TaggableEntity::Place => write!(f, "place"),
        }
    }
}

/// A list of tags to submit for an entity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagList {
    pub tags: Vec<UserTag>,
    pub entity: TaggableEntity,
    pub entity_id: String,
}

impl TagList {
    pub fn new(entity: TaggableEntity, entity_id: String) -> Self {
        Self {
            tags: Vec::new(),
            entity,
            entity_id,
        }
    }

    pub fn add_tag(&mut self, name: String, vote: TagVote) {
        self.tags.push(UserTag { name, vote });
    }

    pub fn to_body_xml(&self) -> String {
        let tags = self
            .tags
            .iter()
            .map(|tag| tag.to_body_xml())
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"<{entity}-list><{entity} id="{id}"><user-tag-list>{tags}</user-tag-list></{entity}></{entity}-list>"#,
            entity = self.entity,
            id = self.entity_id,
        )
    }

    pub async fn send(
        client: &MusicBrainzClient,
        token: &str,
        tags: &[Self],
    ) -> Result<(), crate::Error> {
        let body = tags
            .iter()
            .map(|taglist| taglist.to_body_xml())
            .collect::<Vec<_>>()
            .join("\n");

        client
            .post(
                token,
                "ws/2/tag",
                &format!(
                    "<metadata xmlns=\"http://musicbrainz.org/ns/mmd-2.0#\">{body}</metadata>"
                ),
            )
            .await
    }
}
