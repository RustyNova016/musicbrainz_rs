use core::marker::PhantomData;

use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::endpoints::UriBuilderError;
use api_bindium::ureq::http::Uri;
use api_bindium::ApiRequest;
use serde::de::DeserializeOwned;

use crate::api::query::Query;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::api::ApiEndpointError;
use crate::entity::search::SearchResult;
use crate::entity::search::Searchable;
use crate::APIPath;
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
///     let query_result = Artist::search(query).execute_async().await?;
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

    // === Request Creation ===

    fn create_url(&self, client: &MusicBrainzClient) -> Result<Uri, UriBuilderError> {
        let mut url = self.inner.get_endpoint(client);

        // lucene_query_builder returns a premade leading "query=". We need to remove it first
        // https://github.com/oknozor/lucene_query_builder_rs/issues/3

        url = url.add_parameter("query", &self.search_query);

        url = url.maybe_add_parameter("limit", self.limit.as_ref());
        url = url.maybe_add_parameter("offset", self.offset.as_ref());

        url.to_uri()
    }

    /// Turn the query into an [`api_bindium::ApiRequest`]
    pub fn as_api_request(
        &self,
        client: &crate::MusicBrainzClient,
    ) -> Result<ApiRequest<JsonParser<SearchResult<T>>>, UriBuilderError>
    where
        T: Searchable + DeserializeOwned,
    {
        Ok(ApiRequest::builder()
            .uri(self.create_url(client)?)
            .verb(api_bindium::HTTPVerb::Get)
            .build())
    }

    #[cfg(feature = "sync")]
    pub fn execute(&mut self) -> Result<SearchResult<T>, ApiEndpointError>
    where
        T: Search + Searchable + DeserializeOwned + Sync,
    {
        use crate::client::MUSICBRAINZ_CLIENT;

        self.execute_with_client(&MUSICBRAINZ_CLIENT)
    }

    /// Execute the query with a specific client
    #[cfg(feature = "sync")]
    pub fn execute_with_client(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<SearchResult<T>, ApiEndpointError>
    where
        T: Search + Searchable + DeserializeOwned + Sync,
    {
        use snafu::ResultExt;

        use crate::api::ApiRequestSnafu;
        use crate::api::InvalidUriSnafu;

        self.as_api_request(client)
            .context(InvalidUriSnafu)?
            .send(&client.api_client)
            .context(ApiRequestSnafu)
    }

    #[cfg(feature = "async")]
    pub async fn execute_async(&mut self) -> Result<SearchResult<T>, ApiEndpointError>
    where
        T: Search + Searchable + DeserializeOwned + Sync,
    {
        use crate::client::MUSICBRAINZ_CLIENT;

        self.execute_with_client_async(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[cfg(feature = "async")]
    pub async fn execute_with_client_async(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<SearchResult<T>, ApiEndpointError>
    where
        T: Search + Searchable + DeserializeOwned + Sync,
    {
        use snafu::ResultExt;

        use crate::api::ApiRequestSnafu;
        use crate::api::InvalidUriSnafu;

        self.as_api_request(client)
            .context(InvalidUriSnafu)?
            .send_async(&client.api_client)
            .await
            .context(ApiRequestSnafu)
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
