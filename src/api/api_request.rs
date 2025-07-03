#[cfg(feature = "blocking")]
use reqwest::blocking::Response;
#[cfg(feature = "async")]
use reqwest::Response;
use serde::de::DeserializeOwned;
use tracing::debug;

use crate::client::MusicBrainzClient;
use crate::entity::api::MusicbrainzError;

/// A raw API request, used to send custom requests to the API
pub struct ApiRequest {
    /// The url to fetch
    pub url: String,

    /// The current number of times the request has been tried
    pub tries: u32,
}

impl ApiRequest {
    pub fn new(url: String) -> Self {
        Self { url, tries: 0 }
    }

    #[maybe_async::maybe_async]
    /// Send the request and return a raw response
    pub async fn send_raw(&self, client: &MusicBrainzClient) -> Result<Response, reqwest::Error> {
        let http_request = client.reqwest_client.get(&self.url);

        debug!(
            "Sending api request `{}` (attempt: {})",
            self.url, self.tries
        );
        http_request.send().await
    }

    /// Sends a get request to the musicbrainz api. Return a [serde_json::Value]
    #[maybe_async::maybe_async]
    pub async fn get_json(
        self,
        client: &MusicBrainzClient,
    ) -> Result<serde_json::Value, crate::Error> {
        Ok(client
            .send_with_retries(self)
            .await?
            .json::<serde_json::Value>()
            .await?)
    }

    /// Parse a [serde_json::Value] into Musicbrainz structs
    pub fn parse_json<T>(json: serde_json::Value, url: &str) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
    {
        // Try to deserialize as our result
        let err = match serde_json::from_value::<T>(json.clone()) {
            Ok(result) => return Ok(result),
            Err(err) => err,
        };

        // We have an error. Let's try deserializing MB's error
        if let Ok(result) = serde_json::from_value::<MusicbrainzError>(json) {
            return Err(result.into_error(url.to_string()));
        };

        // Not an MB error? Then it's a problem with out models. Let's send the serde error
        Err(err.into())
    }

    /// Send the request as a get, deal with ratelimits, and retries
    #[maybe_async::maybe_async]
    pub async fn get<T>(self, client: &MusicBrainzClient) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
    {
        let url = self.url.to_string();
        let json = self.get_json(client).await?;
        Self::parse_json(json, &url)
    }
}
