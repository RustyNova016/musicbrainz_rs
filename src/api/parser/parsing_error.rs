use snafu::Snafu;

use crate::entity::api::MusicbrainzError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ParsingError {
    ApiResponseError {
        source: api_bindium::ApiRequestError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    MusicBrainzError {
        source: MusicbrainzError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}

impl ParsingError {
    // If the underlying error is an [`MusicbrainzError`](crate::entity::api::MusicbrainzError), return it
    pub fn as_musicbrainz_error(&self) -> Option<&MusicbrainzError> {
        match self {
            Self::MusicBrainzError { source, .. } => Some(source),
            _ => None,
        }
    }
}
