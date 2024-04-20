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

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use crate::config::*;

/// Configure the HTTP client global state
pub mod config;
mod deserialization;
/// All Musicbrainz entities
pub mod entity;
/// Brings trait and type needed to perform any API query in scope
pub mod prelude;

use crate::entity::search::{SearchResult, Searchable};
use deserialization::date_format;
use entity::Browsable;
use entity::BrowseResult;
use entity::Include;
use entity::{CoverartResolution, CoverartResponse, CoverartTarget, CoverartType};
use std::fmt::Write as _;

/// Type alias for [reqwest::Error]
pub type Error = reqwest::Error;

#[derive(Clone, Debug)]
struct Query<T> {
    path: String,
    include: Vec<Include>,
    phantom: PhantomData<T>,
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
    offset: Option<u16>,
    limit: Option<u8>,
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
pub struct SearchQuery<T>(Query<T>);

impl<'a, T> FetchQuery<T>
where
    T: Clone,
{
    pub fn id(&mut self, id: &str) -> &mut Self {
        let _ = write!(self.0.path, "/{id}");
        self
    }

    #[cfg(feature = "blocking")]
    pub fn execute(&mut self) -> Result<T, Error>
    where
        T: Fetch<'a> + DeserializeOwned,
    {
        self.0.path.push_str(FMT_JSON);
        self.include_to_path();
        let request = HTTP_CLIENT.get(&self.0.path);
        HTTP_CLIENT.send_with_retries(request)?.json()
    }

    #[cfg(feature = "async")]
    pub async fn execute(&mut self) -> Result<T, Error>
    where
        T: Fetch<'a> + DeserializeOwned,
    {
        self.0.path.push_str(FMT_JSON);
        self.include_to_path();
        let request = HTTP_CLIENT.get(&self.0.path);
        HTTP_CLIENT.send_with_retries(request).await?.json().await
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T> FetchCoverartQuery<T>
where
    T: Clone + FetchCoverart<'a>,
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

    #[cfg(feature = "blocking")]
    pub fn execute(&mut self) -> Result<CoverartResponse, Error> {
        self.validate();
        let request = HTTP_CLIENT.get(&self.0.path);
        let response = HTTP_CLIENT.send_with_retries(request)?;
        let coverart_response = if self.0.target.img_type.is_some() {
            let url = response.url().clone();
            CoverartResponse::Url(url.to_string())
        } else {
            CoverartResponse::Json(response.json().unwrap())
        };
        Ok(coverart_response)
    }

    #[cfg(feature = "async")]
    pub async fn execute(&mut self) -> Result<CoverartResponse, Error> {
        self.validate();
        let request = HTTP_CLIENT.get(&self.0.path);
        let response = HTTP_CLIENT.send_with_retries(request).await?;
        let coverart_response = if self.0.target.img_type.is_some() {
            let url = response.url().clone();
            CoverartResponse::Url(url.to_string())
        } else {
            CoverartResponse::Json(response.json().await.unwrap())
        };
        Ok(coverart_response)
    }
}

impl<'a, T> BrowseQuery<T>
where
    T: Clone,
{
    #[cfg(feature = "blocking")]
    pub fn execute(&mut self) -> Result<BrowseResult<T>, Error>
    where
        T: Fetch<'a> + DeserializeOwned + Browsable,
    {
        self.include_to_path();
        let request = HTTP_CLIENT.get(&self.inner.path);
        HTTP_CLIENT.send_with_retries(request)?.json()
    }

    #[cfg(feature = "async")]
    pub async fn execute(&mut self) -> Result<BrowseResult<T>, Error>
    where
        T: Fetch<'a> + DeserializeOwned + Browsable,
    {
        self.include_to_path();
        let request = HTTP_CLIENT.get(&self.inner.path);
        HTTP_CLIENT.send_with_retries(request).await?.json().await
    }

    fn include_to_path(&mut self) {
        self.inner.include_to_path();
        if let Some(limit) = self.limit {
            self.inner.path.push_str(PARAM_LIMIT);
            self.inner.path.push_str(&limit.to_string());
        }
        if let Some(offset) = self.offset {
            self.inner.path.push_str(PARAM_OFFSET);
            self.inner.path.push_str(&offset.to_string());
        }
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

impl<'a, T> SearchQuery<T>
where
    T: Search<'a> + Clone,
{
    #[cfg(feature = "blocking")]
    pub fn execute(&mut self) -> Result<SearchResult<T>, Error>
    where
        T: Search<'a> + DeserializeOwned + Searchable,
    {
        self.include_to_path();
        let request = HTTP_CLIENT.get(&self.0.path);
        HTTP_CLIENT.send_with_retries(request)?.json()
    }

    #[cfg(feature = "async")]
    pub async fn execute(&mut self) -> Result<SearchResult<T>, Error>
    where
        T: Search<'a> + DeserializeOwned + Searchable,
    {
        self.include_to_path();
        let request = HTTP_CLIENT.get(&self.0.path);
        HTTP_CLIENT.send_with_retries(request).await?.json().await
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<T> Query<T> {
    fn include(&mut self, include: Include) -> &mut Self {
        self.include.push(include);
        self
    }

    fn include_to_path(&mut self) {
        if !self.include.is_empty() {
            self.path.push_str(PARAM_INC);
        }

        for inc in self.include.iter() {
            self.path.push_str(inc.as_str());
            if Some(inc) != self.include.last() {
                self.path.push('+');
            }
        }
    }
}

/// Provide the entity HTTP api path, do not use this trait directly
pub trait Path<'a> {
    fn path() -> &'static str;
}

/// Implemented by all fetchable entities (see [`FetchQuery`])
pub trait Fetch<'a> {
    fn fetch() -> FetchQuery<Self>
    where
        Self: Sized + Path<'a>,
    {
        FetchQuery(Query {
            path: format!("{}/{}", get_base_url(), Self::path()),
            phantom: PhantomData,
            include: vec![],
        })
    }
}

/// Implemented by all fetchable coverart entities (see [`FetchCoverartQuery`])
pub trait FetchCoverart<'a> {
    fn fetch_coverart() -> FetchCoverartQuery<Self>
    where
        Self: Sized + Path<'a>,
    {
        FetchCoverartQuery(CoverartQuery {
            path: format!("{}/{}", BASE_COVERART_URL, Self::path()),
            phantom: PhantomData,
            target: CoverartTarget {
                img_type: None,
                img_res: None,
            },
        })
    }

    fn get_coverart(&self) -> FetchCoverartQuery<Self>
    where
        Self: Sized + Path<'a>,
        Self: Clone,
    {
        FetchCoverartQuery(CoverartQuery {
            path: format!("{}/{}", BASE_COVERART_URL, Self::path()),
            phantom: PhantomData,
            target: CoverartTarget {
                img_type: None,
                img_res: None,
            },
        })
    }
}

/// Implemented by all browsable entities (see [`BrowseQuery`])
pub trait Browse<'a> {
    fn browse() -> BrowseQuery<Self>
    where
        Self: Sized + Path<'a>,
    {
        BrowseQuery {
            inner: Query {
                path: format!("{}/{}", get_base_url(), Self::path()),
                phantom: PhantomData,
                include: vec![],
            },
            limit: None,
            offset: None,
        }
    }
}

/// Implemented by all searchable entities (see [`SearchQuery`])
pub trait Search<'a> {
    fn search(query: String) -> SearchQuery<Self>
    where
        Self: Sized + Path<'a>,
    {
        SearchQuery(Query {
            path: format!("{}/{}{}&{}", get_base_url(), Self::path(), FMT_JSON, query),
            phantom: PhantomData,
            include: vec![],
        })
    }
}
