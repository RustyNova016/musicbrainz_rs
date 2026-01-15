use core::marker::PhantomData;

use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::endpoints::UriBuilderError;
use api_bindium::ureq::http::Uri;
use api_bindium::ApiRequest;
use serde::de::DeserializeOwned;

use crate::api::query::Query;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::api::ApiEndpointError;
use crate::entity::Browsable;
use crate::entity::BrowseResult;
use crate::APIPath;

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
/// # async fn main() -> Result<(), musicbrainz_rs::GetRequestError> {
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
/// # fn main() -> Result<(), musicbrainz_rs::GetRequestError> {
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
    pub inner: Query<T>,

    /// The number of results to offset the query by
    pub offset: Option<u16>,

    /// The number of results to query
    pub limit: Option<u8>,

    /// The type of entity used in the filter
    pub filter_entity: String,

    /// The mbid of the filtered entity
    pub filter_mbid: String,
}

impl<T> BrowseQuery<T>
where
    T: Clone,
{
    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: u16) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    // === Request Creation ===

    /// Create the request's url
    fn create_url(&self, client: &crate::MusicBrainzClient) -> Result<Uri, UriBuilderError> {
        let mut url = self.inner.get_endpoint(client);

        // Add the browse filter
        url = url.add_parameter(&self.filter_entity, &self.filter_mbid);

        url = url.maybe_add_parameter("limit", self.limit.as_ref());
        url = url.maybe_add_parameter("offset", self.offset.as_ref());

        url.to_uri()
    }

    /// Turn the query into an [`api_bindium::ApiRequest`]
    pub fn as_api_request(
        &self,
        client: &crate::MusicBrainzClient,
    ) -> Result<ApiRequest<JsonParser<BrowseResult<T>>>, UriBuilderError>
    where
        T: DeserializeOwned + Browsable,
    {
        Ok(ApiRequest::builder()
            .uri(self.create_url(client)?)
            .verb(api_bindium::HTTPVerb::Get)
            .build())
    }

    // === Api Fetching ===

    #[cfg(feature = "sync")]
    pub fn execute(&mut self) -> Result<BrowseResult<T>, ApiEndpointError>
    where
        T: Browse + Browsable + DeserializeOwned + Sync,
    {
        use crate::client::MUSICBRAINZ_CLIENT;

        self.execute_with_client(&MUSICBRAINZ_CLIENT)
    }

    /// Execute the query with a specific client
    #[cfg(feature = "sync")]
    pub fn execute_with_client(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<BrowseResult<T>, ApiEndpointError>
    where
        T: Browse + Browsable + DeserializeOwned + Sync,
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
    pub async fn execute_async(&mut self) -> Result<BrowseResult<T>, ApiEndpointError>
    where
        T: Browse + Browsable + DeserializeOwned + Sync,
    {
        use crate::client::MUSICBRAINZ_CLIENT;

        self.execute_with_client_async(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[cfg(feature = "async")]
    pub async fn execute_with_client_async(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<BrowseResult<T>, ApiEndpointError>
    where
        T: Browse + Browsable + DeserializeOwned + Sync,
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

/// Implemented by all browsable entities (see [`BrowseQuery`])
pub trait Browse {
    fn browse() -> BrowseQuery<Self>
    where
        Self: Sized + APIPath,
    {
        BrowseQuery {
            inner: Query {
                path: Self::path().to_string(),
                result_type: PhantomData,
                include: vec![],
            },
            limit: None,
            offset: None,
            filter_entity: String::new(),
            filter_mbid: String::new(),
        }
    }
}
