//! MusicBrainz rust is a utility crate for the the
//! [MusicBrainz API](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2).
//! It strives to provide a simple and easy to use API to query the Musicbrainz database.
//!
//! All query are performed via a builder pattern fashioned syntax on musicbrainz entities available
//! in the [`entity`] module.
//!
//! ## Example
//!
//! The most simple usage would be to lookup an entity, knowing its [Musicbrainz ID](https://musicbrainz.org/doc/MusicBrainz_Identifier).
//!
//!  ```rust
//! use musicbrainz_rs::entity::artist::Artist;
//! use musicbrainz_rs::prelude::*;
//!
//! # #[cfg(feature = "async")]
//! #[tokio::main]
//! async fn main() -> Result<(), musicbrainz_rs::GetRequestError> {
//!
//!     let nirvana = Artist::fetch()
//!         .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
//!         .execute()
//!          .await;
//!
//!     assert_eq!(nirvana?.name, "Nirvana".to_string());
//!     Ok(())
//! }
//! # #[cfg(feature = "blocking")]
//! fn main() -> Result<(), musicbrainz_rs::GetRequestError> {
//!
//!     let nirvana = Artist::fetch()
//!         .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
//!         .execute();
//!
//!     assert_eq!(nirvana?.name, "Nirvana".to_string());
//!     Ok(())
//! }
//! ```
//!
//! Note that you need to either directly to bring the [`Fetch`] trait in scope or use the
//! [`prelude`] module to make the fetch method accessible.
//!
//! [musicbrainz::prelude]: crate::prelude
//! [entity]: crate::entity

#![allow(clippy::result_large_err)]

/// All the configurations for API queries / fetching
pub mod api;

/// The request clien
pub mod client;

/// Configure the HTTP client global state
pub mod config;

/// The deserializers for the specific Musicbrainz responses
mod deserialization;

/// All Musicbrainz entities
pub mod entity;
/// Module for error reexports
pub mod error;
pub mod extra_endpoints;

/// Brings trait and type needed to perform any API query in scope
pub mod prelude;
#[cfg(feature = "extras")]
pub mod utils;

/// Extra utilities that aren't strictly related to the API
// === Re-exports ===
pub use crate::api::api_request::ApiRequest;
pub use crate::api::api_request::GetRequestError;
pub use crate::api::browse_query::Browse;
pub use crate::api::browse_query::BrowseQuery;
pub use crate::api::coverart_query::FetchCoverart;
pub use crate::api::coverart_query::FetchCoverartQuery;
pub use crate::api::fetch_query::Fetch;
pub use crate::api::fetch_query::FetchQuery;
pub use crate::api::search_query::Search;
pub use crate::api::search_query::SearchQuery;
pub use crate::client::MusicBrainzClient;

pub(crate) use crate::api::coverart_query::CoverartQuery;

/// Chrono Re-export
pub mod chrono {
    pub use chrono::*;
}

#[allow(unused_imports)]
pub(crate) mod reqwester {
    #[cfg(feature = "async")]
    pub use reqwest::{Client as ReqwestClient, RequestBuilder, Response};

    #[cfg(feature = "blocking")]
    pub use reqwest::blocking::{Client as ReqwestClient, RequestBuilder, Response};
}

/// Provide the entity HTTP api path, do not use this trait directly
pub trait APIPath {
    fn path() -> &'static str;
}
