use core::error::Error;
use core::time::Duration;

#[cfg(feature = "blocking")]
use reqwest::blocking::Response;
#[cfg(feature = "async")]
use reqwest::Response;
use snafu::ResultExt as _;
#[cfg(feature = "blocking")]
use std::thread::sleep;

#[cfg(feature = "async")]
use tokio::time::sleep;
use tracing::debug;

use crate::config::HTTP_RATELIMIT_CODE;
use crate::ApiRequest;
use crate::MusicBrainzClient;

impl ApiRequest {
    #[maybe_async::maybe_async]
    /// Send the request and return a raw response
    pub async fn send_raw(&self, client: &MusicBrainzClient) -> Result<Response, RequestSendError> {
        let http_request = client.reqwest_client.get(&self.url);

        debug!(
            "Sending api request `{}` (attempt: {})",
            self.url, self.tries
        );
        http_request.send().await.context(RequestSendSnafu)
    }

    /// Try sending the request.
    #[maybe_async::maybe_async]
    pub async fn try_send(
        &mut self,
        client: &MusicBrainzClient,
    ) -> Result<Option<Response>, RequestSendError> {
        self.tries += 1;

        let response = match self.send_raw(client).await {
            Ok(val) => val,
            Err(err) => {
                if err.is_retryable() {
                    return Ok(None);
                } else {
                    return Err(err);
                }
            }
        };

        // Let's check if we hit the rate limit
        if response.status().as_u16() == HTTP_RATELIMIT_CODE {
            // Oh no. Let's wait the timeout
            let headers = response.headers();
            let retry_secs = headers.get("retry-after").unwrap().to_str().unwrap();
            let duration = Duration::from_secs(retry_secs.parse::<u64>().unwrap() + 1);
            sleep(duration).await;

            return Ok(None);
        }

        Ok(Some(response))
    }

    /// Send the reqwest, deal with ratelimits, and retries
    #[maybe_async::maybe_async]
    pub(crate) async fn send_with_retries(
        &mut self,
        client: &MusicBrainzClient,
    ) -> Result<Response, SendWithRetriesError> {
        while self.tries < client.max_retries {
            client.wait_for_ratelimit().await;

            if let Some(res) = self.try_send(client).await.context(RequestSendFatalSnafu)? {
                return Ok(res);
            }
        }

        MaxRetriesExceededSnafu.fail()
    }
}

/// Error for the [`ApiRequest::send_raw`] function
#[derive(Debug, snafu::Snafu)]
#[snafu(display("Couldn't successfully send the http request"))]
pub struct RequestSendError {
    source: reqwest::Error,

    #[cfg(feature = "backtrace")]
    backtrace: snafu::Backtrace,
}

impl RequestSendError {
    /// Return true if the error is temporary and should be retried
    pub fn is_retryable(&self) -> bool {
        self.source.is_timeout() || self.is_connection_reset()
    }

    pub fn is_connection_reset(&self) -> bool {
        // Hyper_util error
        let Some(source) = self.source.source() else {
            return false;
        };
        let Some(hyper_util_error) = source.downcast_ref::<hyper_util::client::legacy::Error>()
        else {
            return false;
        };

        // Hyper error
        let Some(source) = hyper_util_error.source() else {
            return false;
        };
        let Some(hyper_error) = source.downcast_ref::<hyper::Error>() else {
            return false;
        };

        // IO error
        let Some(source) = hyper_error.source() else {
            return false;
        };
        let Some(std_error) = source.downcast_ref::<std::io::Error>() else {
            return false;
        };

        std_error.kind() == std::io::ErrorKind::ConnectionReset
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum SendWithRetriesError {
    #[snafu(display("Couldn't successfully send the http request with retries"))]
    RequestSendFatalError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: RequestSendError,
    },

    #[snafu(display("The max retry count for the request as been exeeded. You may want to check if the correct url is set, musicbrainz is online, or you aren't hitting the ratelimit."))]
    MaxRetriesExceeded {
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}
