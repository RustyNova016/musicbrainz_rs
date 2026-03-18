use api_bindium::ApiRequestError;
use api_bindium::endpoints::UriBuilderError;
use snafu::Snafu;

use crate::api::parser::ParsingError;

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
