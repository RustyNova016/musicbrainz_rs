use api_bindium::ApiRequestError;
use api_bindium::endpoints::UriBuilderError;
use snafu::Snafu;

use crate::ParsingError;
use crate::entity::api::MusicbrainzError;

pub mod browse_query;
pub mod coverart_query;
pub mod endpoints;
pub mod fetch_query;
pub mod impl_browse_includes;
pub mod impl_relations_includes;
pub mod parser;
pub mod query;
pub mod search_query;
pub mod ws;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum ApiEndpointError {
    ApiRequestError {
        source: ApiRequestError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    InvalidUriError {
        source: UriBuilderError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    ParsingError {
        source: ParsingError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}

impl ApiEndpointError {
    // If the underlying error is an [`MusicbrainzError`](crate::entity::api::MusicbrainzError), return it
    pub fn as_musicbrainz_error(&self) -> Option<&MusicbrainzError> {
        match self {
            Self::ParsingError { source, .. } => source.as_musicbrainz_error(),
            Self::ApiRequestError { .. } | Self::InvalidUriError { .. } => None,
        }
    }
}
