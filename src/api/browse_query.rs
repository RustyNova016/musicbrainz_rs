use core::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::api::api_request::GetRequestError;
use crate::api::fetch_query::Fetch;
use crate::api::query::Query;
use crate::client::MUSICBRAINZ_CLIENT;
use crate::config::PARAM_LIMIT;
use crate::config::PARAM_OFFSET;
use crate::entity::Browsable;
use crate::entity::BrowseResult;
use crate::APIPath;
use crate::ApiRequest;

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

    /// The search query
    pub id: String,
}

impl<T> BrowseQuery<T>
where
    T: Clone,
{
    #[maybe_async::maybe_async]
    pub async fn execute(&mut self) -> Result<BrowseResult<T>, GetRequestError>
    where
        T: Fetch + DeserializeOwned + Browsable,
    {
        self.execute_with_client(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[maybe_async::maybe_async]
    pub async fn execute_with_client(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<BrowseResult<T>, GetRequestError>
    where
        T: Fetch + DeserializeOwned + Browsable,
    {
        self.as_api_request(client).get(client).await
    }

    fn create_url(&self, client: &crate::MusicBrainzClient) -> String {
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

    /// Turn the query into an [`crate::ApiRequest`]
    pub fn as_api_request(&self, client: &crate::MusicBrainzClient) -> ApiRequest {
        ApiRequest::new(self.create_url(client))
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
            id: String::new(),
        }
    }
}
