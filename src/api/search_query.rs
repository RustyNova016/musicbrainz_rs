use core::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::api::api_request::GetRequestError;
use crate::api::query::Query;
use crate::client::MUSICBRAINZ_CLIENT;
use crate::config::PARAM_LIMIT;
use crate::config::PARAM_OFFSET;
use crate::entity::search::SearchResult;
use crate::entity::search::Searchable;
use crate::APIPath;
use crate::ApiRequest;
use crate::MusicBrainzClient;

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
/// # async fn main() -> Result<(), musicbrainz_rs::GetRequestError> {
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
/// # fn main() -> Result<(), musicbrainz_rs::GetRequestError> {
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
    pub inner: Query<T>,

    /// The number of results to offset the query by
    pub offset: Option<u16>,

    /// The number of results to query
    pub limit: Option<u8>,

    /// The search query in lucene
    pub search_query: String,
}

impl<T> SearchQuery<T>
where
    T: Search + Clone,
{
    /// Turn the query into an [`crate::ApiRequest`]
    pub fn as_api_request(&self, client: &crate::MusicBrainzClient) -> ApiRequest {
        ApiRequest::new(self.create_url(client))
    }

    #[maybe_async::maybe_async]
    pub async fn execute(&mut self) -> Result<SearchResult<T>, GetRequestError>
    where
        T: Search + DeserializeOwned + Searchable,
    {
        self.execute_with_client(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[maybe_async::maybe_async]
    pub async fn execute_with_client(
        &mut self,
        client: &MusicBrainzClient,
    ) -> Result<SearchResult<T>, GetRequestError>
    where
        T: Search + DeserializeOwned + Searchable,
    {
        self.as_api_request(client).get(client).await
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

/// Implemented by all searchable entities (see [`SearchQuery`])
pub trait Search {
    fn search(query: String) -> SearchQuery<Self>
    where
        Self: Sized + APIPath,
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
