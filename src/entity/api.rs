use core::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

/// An error given by musicbrainz's API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MusicbrainzError {
    error: String,
    help: String,
}

impl MusicbrainzError {
    pub fn is_not_found(&self) -> bool {
        self.error == "Not Found"
    }
}

impl Display for MusicbrainzError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: {}\nHelp: {}", self.error, self.help)
    }
}

impl core::error::Error for MusicbrainzError {}
