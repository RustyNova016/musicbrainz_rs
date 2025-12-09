use api_bindium::endpoints::UriBuilderError;
use api_bindium::ApiRequestError;
use snafu::Snafu;

pub mod browse_query;
pub mod coverart_query;
pub mod fetch_query;
pub mod impl_browse_includes;
pub mod impl_relations_includes;
pub mod query;
pub mod search_query;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum ApiEndpointError {
    ApiRequestError {
        source: ApiRequestError,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    InvalidUriError {
        source: UriBuilderError,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}
