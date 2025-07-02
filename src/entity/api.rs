use serde::Deserialize;
use serde::Serialize;

use crate::Error;

/// An error given by musicbrainz's API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MusicbrainzError {
    error: String,
    help: String,
}

impl MusicbrainzError {
    pub fn into_error(self, query: String) -> Error {
        if self.is_not_found() {
            return Error::NotFound(query);
        }

        Error::MusicbrainzError(query, self)
    }

    pub fn is_not_found(&self) -> bool {
        self.error == "Not Found"
    }
}
