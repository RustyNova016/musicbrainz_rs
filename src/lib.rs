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
//! # #[cfg(feature = "sync")]
//! fn main() -> Result<(), musicbrainz_rs::ApiEndpointError> {
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

/// The api endpoint makers
pub mod api;
/// The request client
pub mod client;
/// The deserializers for the specific Musicbrainz responses
mod deserialization;
/// All Musicbrainz entities
pub mod entity;
/// Brings trait and type needed to perform any API query in scope
pub mod prelude;
/// Extra utilities that aren't strictly related to the API
#[cfg(feature = "extras")]
pub mod utils;

// === Re-exports ===
pub use crate::api::ApiEndpointError;
pub use crate::api::browse_query::Browse;
pub use crate::api::browse_query::BrowseQuery;
pub use crate::api::coverart_query::CoverartQuery;
pub use crate::api::coverart_query::FetchCoverart;
pub use crate::api::coverart_query::FetchCoverartQuery;
pub use crate::api::fetch_query::Fetch;
pub use crate::api::fetch_query::FetchQuery;
pub use crate::api::search_query::Search;
pub use crate::api::search_query::SearchQuery;
pub use crate::client::MusicBrainzClient;

// === Crate Reexports ==

pub use api_bindium;
pub use chrono;

/// Provide the entity HTTP api path, do not use this trait directly
pub trait APIPath {
    fn path() -> &'static str;
}
