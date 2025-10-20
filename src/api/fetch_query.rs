use core::fmt::Write as _;
use core::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::api::api_request::GetRequestError;
use crate::api::query::Query;
use crate::client::MUSICBRAINZ_CLIENT;
use crate::APIPath;
use crate::ApiRequest;

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
/// # #[cfg(feature = "blocking")]
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

    /// Turn the query into an [`crate::ApiRequest`]
    pub fn as_api_request(&self, client: &crate::MusicBrainzClient) -> ApiRequest {
        ApiRequest::new(self.0.create_url(client))
    }

    #[maybe_async::maybe_async]
    pub async fn execute(&mut self) -> Result<T, GetRequestError>
    where
        T: Fetch + DeserializeOwned,
    {
        self.execute_with_client(&MUSICBRAINZ_CLIENT).await
    }

    /// Execute the query with a specific client
    #[maybe_async::maybe_async]
    pub async fn execute_with_client(
        &mut self,
        client: &crate::MusicBrainzClient,
    ) -> Result<T, GetRequestError>
    where
        T: Fetch + DeserializeOwned,
    {
        self.as_api_request(client).get(client).await
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
