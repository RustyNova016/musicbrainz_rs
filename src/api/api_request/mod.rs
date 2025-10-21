pub mod fetching;

use serde::de::DeserializeOwned;
use snafu::ResultExt;

use crate::api::api_request::fetching::SendWithRetriesError;
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

    /// Sends a get request to the musicbrainz api. Return a [serde_json::Value]
    #[maybe_async::maybe_async]
    pub async fn get_json(
        mut self,
        client: &MusicBrainzClient,
    ) -> Result<serde_json::Value, RequestJsonError> {
        let request = self
            .send_with_retries(client)
            .await
            .context(SendWithRetriesSnafu)?;

        let text = request.text().await.context(ResponseDataSnafu)?;
        let json: serde_json::Value =
            serde_json::from_str(&text).with_context(|_| JsonParseSnafu {
                data: text.to_owned(),
            })?;

        Ok(json)
    }

    /// Parse a [serde_json::Value] into Musicbrainz structs
    pub fn parse_json<T>(json: serde_json::Value) -> Result<T, RequestJsonParsingError>
    where
        T: DeserializeOwned,
    {
        // Try to deserialize as our result
        let err = match serde_json::from_value::<T>(json.clone()) {
            Ok(result) => return Ok(result),
            Err(err) => err,
        };

        // We have an error. Let's try deserializing MB's error
        if let Ok(result) = serde_json::from_value::<MusicbrainzError>(json.clone()) {
            return Err(result).context(ApiSnafu);
        };

        // Not an MB error? Then it's a problem with our models. Let's send the serde error
        Err(err).with_context(|_| InvalidResponseSnafu {
            data: json.to_owned(),
        })
    }

    /// Send the request as a get, deal with ratelimits, and retries
    #[maybe_async::maybe_async]
    pub async fn get<T>(self, client: &MusicBrainzClient) -> Result<T, GetRequestError>
    where
        T: DeserializeOwned,
    {
        let url = self.url.to_string();
        let json = self.get_json(client).await.with_context(|_| RequestSnafu {
            url: url.to_owned(),
        })?;
        Self::parse_json(json).with_context(|_| ParsingSnafu {
            url: url.to_owned(),
        })
    }
}

/// Error for the [`ApiRequest::get_json`] function
#[derive(Debug, snafu::Snafu)]
pub enum RequestJsonError {
    #[snafu(display("Couldn't get the json data for the request"))]
    SendWithRetriesError {
        source: SendWithRetriesError,
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(display("Couldn't retrieve the response data"))]
    ResponseDataError {
        source: reqwest::Error,
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(display("Couldn't parse the text as json:\n{data}"))]
    JsonParseError {
        source: serde_json::Error,
        data: String,
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}

/// Error for the [`ApiRequest::parse_json`] function
#[derive(Debug, snafu::Snafu)]
pub enum RequestJsonParsingError {
    #[snafu(display("The api's response couldn't be deserialized:\n{data}"))]
    InvalidResponse {
        source: serde_json::Error,
        data: serde_json::Value,
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(display("The API returned an error"))]
    ApiError {
        source: MusicbrainzError,
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}

/// Error for the [`ApiRequest::get`] function
#[derive(Debug, snafu::Snafu)]
pub enum GetRequestError {
    #[snafu(display("An error happened while requesting url:\n{url}"))]
    RequestError {
        source: RequestJsonError,
        url: String,
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(display("An error happened while parsing the responce of url:\n{url}"))]
    ParsingError {
        source: RequestJsonParsingError,
        url: String,
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}
