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
//! async fn main() -> Result<(), Error> {
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
//! fn main() -> Result<(), Error> {
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
//! [musicbrainz::prelude]: musicbrainz_rs::prelude
//! [entity]: musicbrainz_rs::entity

use query::Query;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use crate::config::*;

/// Configure the HTTP client global state
pub mod config;

/// Configure the HTTP client global state
pub mod client;

/// The deserializers for the specific Musicbrainz responses
mod deserialization;

/// All Musicbrainz entities
pub mod entity;

/// Brings trait and type needed to perform any API query in scope
pub mod prelude;

/// The structures to create queries
pub mod query;

/// Crate errors;
pub mod error;

pub mod extra_endpoints;

/// Extra utilities that aren't strictly related to the API
#[cfg(feature = "extras")]
pub mod utils;

use crate::entity::search::{SearchResult, Searchable};
use client::MusicBrainzClient;
use client::MUSICBRAINZ_CLIENT;
use deserialization::date_format;
use entity::Browsable;
use entity::BrowseResult;
use entity::{CoverartResolution, CoverartResponse, CoverartTarget, CoverartType};
use std::fmt::Write as _;

/// Rexports
pub use crate::error::Error;

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

/// perform a lookup of an entity when you have the MBID for that entity
///
/// # Lookups
///
/// You can perform a lookup of an entity when you have the MBID for that entity.
///
/// ## EXample
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # #[tokio::main]
/// # #[cfg(feature = "async")]
/// # async fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::Artist;
/// let nirvana = Artist::fetch()
///         .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
///         .execute()
///         .await;
///
/// assert_eq!(nirvana?.name, "Nirvana".to_string());
/// #   Ok(())
/// # }
/// # #[cfg(feature = "blocking")]
/// # fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::Artist;
/// let nirvana = Artist::fetch()
///         .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
///         .execute();
///
/// assert_eq!(nirvana?.name, "Nirvana".to_string());
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct FetchQuery<T>(Query<T>);

/// perform a lookup of an entity's coverart when you have the MBID for that entity
///
/// # Lookups
///
/// You can perform a lookup of an entity's coverart when you have the MBID for that entity.
///
/// ## Example
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # #[tokio::main]
/// # #[cfg(feature = "async")]
/// # async fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::release::Release;
/// # use musicbrainz_rs::entity::CoverartResponse;
/// let in_utero_coverart = Release::fetch_coverart()
///         .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
///         .execute()
///         .await?;
///
/// if let CoverartResponse::Json(coverart) = in_utero_coverart {
///     assert_eq!(coverart.images[0].front, true);
///     assert_eq!(coverart.images[0].back, false);
/// } else {
///     assert!(false);
/// }
/// #   Ok(())
/// # }
/// # #[cfg(feature = "blocking")]
/// # fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::release::Release;
/// # use musicbrainz_rs::entity::CoverartResponse;
/// let in_utero_coverart = Release::fetch_coverart()
///         .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
///         .execute()?;
///
/// if let CoverartResponse::Json(coverart) = in_utero_coverart {
///     assert_eq!(coverart.images[0].front, true);
///     assert_eq!(coverart.images[0].back, false);
/// } else {
///     assert!(false);
/// }
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
struct CoverartQuery<T> {
    path: String,
    target: CoverartTarget,
    phantom: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct FetchCoverartQuery<T>(CoverartQuery<T>);

/// Direct lookup of all the entities directly linked to another entity
///
/// # Browse
///
/// Browse requests are a direct lookup of all the entities directly linked to another entity
/// ("directly linked" here meaning it does not include entities linked by a relationship).
///
/// ## Example
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # #[tokio::main]
/// # #[cfg(feature = "async")]
/// # async fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::Artist;
/// # use musicbrainz_rs::entity::release::Release;
/// let ubiktune_releases = Release::browse()
///         .by_label("47e718e1-7ee4-460c-b1cc-1192a841c6e5")
///         .execute()
///         .await;
///
/// assert!(!ubiktune_releases?.entities.is_empty());
/// #   Ok(())
/// # }
/// # #[cfg(feature = "blocking")]
/// # fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::Artist;
/// # use musicbrainz_rs::entity::release::Release;
/// let ubiktune_releases = Release::browse()
///         .by_label("47e718e1-7ee4-460c-b1cc-1192a841c6e5")
///         .execute();
///
/// assert!(!ubiktune_releases?.entities.is_empty());
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct BrowseQuery<T> {
    inner: Query<T>,

    /// The number of results to offset the query by
    offset: Option<u16>,

    /// The number of results to query
    limit: Option<u8>,

    /// The search query
    id: String,
}

/// Search requests provide a way to search for MusicBrainz entities based on different
/// sorts of queries.
///
///# Search
///
/// The MusicBrainz API search requests provide a way to search for MusicBrainz entities
/// based on different sorts of queries.
/// ## Example
///
///```rust
/// # use musicbrainz_rs::prelude::*;
/// # #[tokio::main]
/// # #[cfg(feature = "async")]
/// # async fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::{Artist, ArtistSearchQuery};
/// let query = ArtistSearchQuery::query_builder()
///         .artist("Miles Davis")
///         .and()
///         .country("US")
///         .build();
///
///     let query_result = Artist::search(query).execute().await?;
///     let query_result: Vec<String> = query_result
///         .entities
///         .iter()
///         .map(|artist| artist.name.clone())
///         .collect();
///
///     assert!(query_result.contains(&"Miles Davis".to_string()));
///     assert!(query_result.contains(&"Miles Davis Quintet".to_string()));
/// #   Ok(())
/// # }
/// # #[cfg(feature = "blocking")]
/// # fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::{Artist, ArtistSearchQuery};
/// let query = ArtistSearchQuery::query_builder()
///         .artist("Miles Davis")
///         .and()
///         .country("US")
///         .build();
///
///     let query_result = Artist::search(query).execute()?;
///     let query_result: Vec<String> = query_result
///         .entities
///         .iter()
///         .map(|artist| artist.name.clone())
///         .collect();
///
///     assert!(query_result.contains(&"Miles Davis".to_string()));
///     assert!(query_result.contains(&"Miles Davis Quintet".to_string()));
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct SearchQuery<T> {
    inner: Query<T>,

    /// The number of results to offset the query by
    offset: Option<u16>,

    /// The number of results to query
    limit: Option<u8>,

    /// The search query in lucene
    search_query: String,
}

impl<T> FetchQuery<T>
where
    T: Clone,
{
    /// The mbid of the entity to fetch
    pub fn id(&mut self, id: &str) -> &mut Self {
        let _ = write!(self.0.path, "/{id}");
        self
    }

    #[maybe_async::maybe_async]
    pub async fn execute(&mut self) -> Result<T, Error>
    where
        T: Fetch + DeserializeOwned,
    {
        self.execute_with_client(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[maybe_async::maybe_async]
    pub async fn execute_with_client(
        &mut self,
        client: &client::MusicBrainzClient,
    ) -> Result<T, Error>
    where
        T: Fetch + DeserializeOwned,
    {
        client.get(&self.0.create_url(client)).await
    }
}

impl<T> FetchCoverartQuery<T>
where
    T: Clone + FetchCoverart,
{
    pub fn id(&mut self, id: &str) -> &mut Self {
        let _ = write!(self.0.path, "/{id}");
        self
    }

    pub fn front(&mut self) -> &mut Self {
        if self.0.target.img_type.is_some() {
            println!("ignoring call to `front`, since coverart type has already been set");
        }
        self.0.target.img_type = Some(CoverartType::Front);
        self
    }

    pub fn back(&mut self) -> &mut Self {
        if self.0.target.img_type.is_some() {
            println!("ignoring call to `back`, since coverart type has already been set");
        }
        self.0.target.img_type = Some(CoverartType::Back);
        self
    }

    pub fn res_250(&mut self) -> &mut Self {
        if self.0.target.img_res.is_some() {
            println!("ignoring call to `res_250`, since resolution has already been set");
        }
        self.0.target.img_res = Some(CoverartResolution::Res250);
        self
    }

    pub fn res_500(&mut self) -> &mut Self {
        if self.0.target.img_res.is_some() {
            println!("ignoring call to `res_500`, since resolution has already been set");
        }
        self.0.target.img_res = Some(CoverartResolution::Res500);
        self
    }

    pub fn res_1200(&mut self) -> &mut Self {
        if self.0.target.img_res.is_some() {
            println!("ignoring call to `res_1200`, since resolution has already been set");
        }
        self.0.target.img_res = Some(CoverartResolution::Res1200);
        self
    }

    pub fn validate(&mut self) {
        if let Some(img_type) = &self.0.target.img_type {
            let _ = write!(self.0.path, "/{}", img_type.as_str());
            if let Some(img_res) = &self.0.target.img_res {
                let _ = write!(self.0.path, "-{}", img_res.as_str());
            }
        } else if self.0.target.img_res.is_some() {
            // Implicitly assume coverart type as front in the case when resolution is
            // explicitly specified but coverart type is not.
            self.front().validate();
        }
    }

    #[maybe_async::maybe_async]
    pub async fn execute(&mut self) -> Result<CoverartResponse, Error> {
        self.execute_with_client(&MUSICBRAINZ_CLIENT).await
    }

    #[maybe_async::maybe_async]
    pub async fn execute_with_client(
        &mut self,
        client: &MusicBrainzClient,
    ) -> Result<CoverartResponse, Error> {
        self.validate();

        let url = format!("{}/{}", client.coverart_archive_url, &self.0.path);

        let response = client
            .send_with_retries(client.reqwest_client.get(&url))
            .await?;
        let coverart_response = if self.0.target.img_type.is_some() {
            let url = response.url().clone();
            CoverartResponse::Url(url.to_string())
        } else {
            CoverartResponse::Json(response.json().await?)
        };
        Ok(coverart_response)
    }
}

impl<T> BrowseQuery<T>
where
    T: Clone,
{
    #[maybe_async::maybe_async]
    pub async fn execute(&mut self) -> Result<BrowseResult<T>, Error>
    where
        T: Fetch + DeserializeOwned + Browsable,
    {
        self.execute_with_client(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[maybe_async::maybe_async]
    pub async fn execute_with_client(
        &mut self,
        client: &client::MusicBrainzClient,
    ) -> Result<BrowseResult<T>, Error>
    where
        T: Fetch + DeserializeOwned + Browsable,
    {
        client.get(&self.create_url(client)).await
    }

    fn create_url(&self, client: &MusicBrainzClient) -> String {
        let mut url = self.inner.create_url(client);
        url.push_str(&format!("&{}", self.id));

        if let Some(limit) = self.limit {
            url.push_str(PARAM_LIMIT);
            url.push_str(&limit.to_string());
        }
        if let Some(offset) = self.offset {
            url.push_str(PARAM_OFFSET);
            url.push_str(&offset.to_string());
        }

        url
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: u16) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}

impl<T> SearchQuery<T>
where
    T: Search + Clone,
{
    #[maybe_async::maybe_async]
    pub async fn execute(&mut self) -> Result<SearchResult<T>, Error>
    where
        T: Search + DeserializeOwned + Searchable,
    {
        self.execute_with_client(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[maybe_async::maybe_async]
    pub async fn execute_with_client(
        &mut self,
        client: &client::MusicBrainzClient,
    ) -> Result<SearchResult<T>, Error>
    where
        T: Search + DeserializeOwned + Searchable,
    {
        client.get(&self.create_url(client)).await
    }

    fn create_url(&self, client: &MusicBrainzClient) -> String {
        let mut url = self.inner.create_url(client);
        url.push_str(&format!("&{}", self.search_query));

        if let Some(limit) = self.limit {
            url.push_str(PARAM_LIMIT);
            url.push_str(&limit.to_string());
        }
        if let Some(offset) = self.offset {
            url.push_str(PARAM_OFFSET);
            url.push_str(&offset.to_string());
        }

        url
    }

    /// An integer value defining how many entries should be returned. Only values between 1 and 100 (both inclusive) are allowed. If not given, this defaults to 25.
    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Return search results starting at a given offset. Used for paging through more than one page of results.
    pub fn offset(&mut self, offset: u16) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}

/// Provide the entity HTTP api path, do not use this trait directly
pub trait Path {
    fn path() -> &'static str;
}

/// Implemented by all fetchable entities (see [`FetchQuery`])
pub trait Fetch {
    fn fetch() -> FetchQuery<Self>
    where
        Self: Sized + Path,
    {
        FetchQuery(Query {
            path: Self::path().to_string(),
            result_type: PhantomData,
            include: vec![],
        })
    }
}

/// Implemented by all fetchable coverart entities (see [`FetchCoverartQuery`])
pub trait FetchCoverart {
    fn fetch_coverart() -> FetchCoverartQuery<Self>
    where
        Self: Sized + Path,
    {
        FetchCoverartQuery(CoverartQuery {
            path: Self::path().to_string(),
            phantom: PhantomData,
            target: CoverartTarget {
                img_type: None,
                img_res: None,
            },
        })
    }

    fn get_coverart(&self) -> FetchCoverartQuery<Self>
    where
        Self: Sized + Path,
        Self: Clone,
    {
        FetchCoverartQuery(CoverartQuery {
            path: Self::path().to_string(),
            phantom: PhantomData,
            target: CoverartTarget {
                img_type: None,
                img_res: None,
            },
        })
    }
}

/// Implemented by all browsable entities (see [`BrowseQuery`])
pub trait Browse {
    fn browse() -> BrowseQuery<Self>
    where
        Self: Sized + Path,
    {
        BrowseQuery {
            inner: Query {
                path: Self::path().to_string(),
                result_type: PhantomData,
                include: vec![],
            },
            limit: None,
            offset: None,
            id: String::new(),
        }
    }
}

/// Implemented by all searchable entities (see [`SearchQuery`])
pub trait Search {
    fn search(query: String) -> SearchQuery<Self>
    where
        Self: Sized + Path,
    {
        SearchQuery {
            inner: Query {
                path: Self::path().to_string(),
                result_type: PhantomData,
                include: vec![],
            },
            search_query: query,
            limit: None,
            offset: None,
        }
    }
}
