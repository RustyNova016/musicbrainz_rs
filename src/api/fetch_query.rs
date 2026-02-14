use core::fmt::Write as _;
use core::marker::PhantomData;
#[cfg(feature = "basic_auth")]
use std::collections::HashMap;
use api_bindium::endpoints::UriBuilderError;
use api_bindium::ApiRequest;
use serde::de::DeserializeOwned;

use crate::api::parser::MusicBrainzParser;
use crate::api::query::Query;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::api::ApiEndpointError;
use crate::APIPath;
#[cfg(feature = "basic_auth")]
use base64::engine::general_purpose::STANDARD;
#[cfg(feature = "basic_auth")]
use base64::Engine;

/// Perform a lookup of an entity when you have the MBID for that entity
///
/// # Lookups
///
/// You can perform a lookup of an entity when you have the MBID for that entity.
///
/// ## Example
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # #[cfg(feature = "sync")]
/// # fn main() -> Result<(), musicbrainz_rs::ApiEndpointError> {
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
    ) -> Result<ApiRequest<MusicBrainzParser<T>>, UriBuilderError>
    where
        T: DeserializeOwned,
    {
        Ok(ApiRequest::builder()
            .uri(self.0.get_endpoint(client).to_uri()?)
            .maybe_headers(Self::auth_headers(client))
            .verb(api_bindium::HTTPVerb::Get)
            .parser(MusicBrainzParser::default())
            .build())
    }

    #[cfg(feature = "basic_auth")]
    fn auth_headers(client: &crate::MusicBrainzClient) -> Option<HashMap<String, String>> {
        client
            .basic_auth_credentials
            .as_ref()
            .map(|(username, password)| {
                let mut headers = HashMap::new();
                headers.insert(
                    "Authorization".to_string(),
                    format!(
                        "Basic {}",
                        STANDARD.encode(format!("{}:{}", username, password))
                    ),
                );
                headers
            })
    }

    #[cfg(not(feature = "basic_auth"))]
    fn auth_headers() -> Option<HashMap<String, String>> {
        None
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
        use snafu::ResultExt as _;

        use crate::api::ApiRequestSnafu;
        use crate::api::InvalidUriSnafu;
        use crate::api::ParsingSnafu;

        self.as_api_request(client)
            .context(InvalidUriSnafu)?
            .send(&client.api_client)
            .context(ApiRequestSnafu)?
            .parse()
            .context(ParsingSnafu)
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
        use snafu::ResultExt as _;

        use crate::api::ApiRequestSnafu;
        use crate::api::InvalidUriSnafu;
        use crate::api::ParsingSnafu;

        self.as_api_request(client)
            .context(InvalidUriSnafu)?
            .send_async(&client.api_client)
            .await
            .context(ApiRequestSnafu)?
            .parse()
            .context(ParsingSnafu)
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
