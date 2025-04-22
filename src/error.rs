use thiserror::Error;

use crate::entity::api::MusicbrainzError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Musicbrainz returned an unknown error")]
    MusicbrainzError(MusicbrainzError),

    #[error("Musicbrainz returned \"Not found\" for query \"{0}\"")]
    NotFound(String),

    #[error("The max retry count for the request as been exeeded. You may want to check if the correct url is set, musicbrainz is online, or you aren't hitting the ratelimit.")]
    MaxRetriesExceeded,

    #[error("No retry token is saved. Check if the user has been authorized first")]
    MissingRetryToken,
}
