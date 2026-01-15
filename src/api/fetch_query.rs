use core::fmt::Write as _;
use core::marker::PhantomData;

use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::endpoints::UriBuilderError;
use api_bindium::ApiRequest;
use serde::de::DeserializeOwned;

use crate::api::query::Query;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::api::ApiEndpointError;
use crate::APIPath;

/// Perform a lookup of an entity when you have the MBID for that entity
///
/// # Lookups
///
/// You can perform a lookup of an entity when you have the MBID for that entity.
///
/// ## Example
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # #[tokio::main]
/// # #[cfg(feature = "async")]
/// # async fn main() -> Result<(), musicbrainz_rs::GetRequestError> {
/// # use musicbrainz_rs::entity::artist::Artist;
/// let nirvana = Artist::fetch()
///         .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
///         .execute()
///         .await;
///
/// assert_eq!(nirvana?.name, "Nirvana".to_string());
/// #   Ok(())
/// # }
/// # #[cfg(feature = "sync")]
/// # fn main() -> Result<(), musicbrainz_rs::GetRequestError> {
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
pub struct FetchQuery<T>(pub Query<T>);

impl<T> FetchQuery<T>
where
    T: Clone,
{
    /// The mbid of the entity to fetch
    pub fn id(&mut self, id: &str) -> &mut Self {
        let _ = write!(self.0.path, "/{id}");
        self
    }

    // === Request Creation ===

    /// Turn the query into an [`api_bindium::ApiRequest`]    
    pub fn as_api_request(
        &self,
        client: &crate::MusicBrainzClient,
    ) -> Result<ApiRequest<JsonParser<T>>, UriBuilderError>
    where
        T: DeserializeOwned,
    {
        Ok(ApiRequest::builder()
            .uri(self.0.get_endpoint(client).to_uri()?)
            .verb(api_bindium::HTTPVerb::Get)
            .build())
    }

    #[cfg(feature = "sync")]
    pub fn execute(&mut self) -> Result<T, ApiEndpointError>
    where
        T: Fetch + DeserializeOwned + Sync,
    {
        use crate::client::MUSICBRAINZ_CLIENT;

        self.execute_with_client(&MUSICBRAINZ_CLIENT)
    }

    /// Execute the query with a specific client
    #[cfg(feature = "sync")]
    pub fn execute_with_client(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<T, ApiEndpointError>
    where
        T: Fetch + DeserializeOwned + Sync,
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
    pub async fn execute_async(&mut self) -> Result<T, ApiEndpointError>
    where
        T: Fetch + DeserializeOwned + Sync,
    {
        use crate::client::MUSICBRAINZ_CLIENT;

        self.execute_with_client_async(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[cfg(feature = "async")]
    pub async fn execute_with_client_async(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<T, ApiEndpointError>
    where
        T: Fetch + DeserializeOwned + Sync,
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

/// Implemented by all fetchable entities (see [`FetchQuery`])
pub trait Fetch {
    fn fetch() -> FetchQuery<Self>
    where
        Self: Sized + APIPath,
    {
        FetchQuery(Query {
            path: Self::path().to_string(),
            result_type: PhantomData,
            include: vec![],
        })
    }
}
