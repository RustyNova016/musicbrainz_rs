use core::time::Duration;

#[cfg(feature = "blocking")]
use reqwest::blocking::Response;
#[cfg(feature = "async")]
use reqwest::Response;
use snafu::ResultExt;

#[cfg(feature = "blocking")]
use std::thread::sleep;

#[cfg(feature = "async")]
use tokio::time::sleep;

use crate::api::api_request::ApiRequest;
use crate::api::api_request::RequestSendError;
use crate::client::MusicBrainzClient;
use crate::config::HTTP_RATELIMIT_CODE;

impl MusicBrainzClient {
    #[maybe_async::maybe_async]
    pub async fn wait_for_ratelimit(&self) {
        #[cfg(feature = "rate_limit")]
        if let Some(val) = &self.rate_limit {
            val.until_ready().await
        }
    }

    /// Send the reqwest, deal with ratelimits, and retries
    #[maybe_async::maybe_async]
    pub(crate) async fn send_with_retries(
        &self,
        mut request: ApiRequest,
    ) -> Result<Response, SendWithRetriesError> {
        while request.tries < self.max_retries {
            self.wait_for_ratelimit().await;

            // Send the query
            let response = request.send_raw(self).await.context(RequestSendSnafu)?;

            // Let's check if we hit the rate limit
            if response.status().as_u16() == HTTP_RATELIMIT_CODE {
                // Oh no. Let's wait the timeout
                let headers = response.headers();
                let retry_secs = headers.get("retry-after").unwrap().to_str().unwrap();
                let duration = Duration::from_secs(retry_secs.parse::<u64>().unwrap() + 1);
                sleep(duration).await;
                request.tries += 1;

                // Hard crash if the rate limit is hit while testing.
                // It should be unacceptable to let the users hit it while we got a fancy system for it
                #[cfg(all(test, feature = "rate_limit"))]
                if self.rate_limit.is_some() {
                    panic!("Rate limit hit on rate limit feature!");
                }
            } else {
                return Ok(response);
            }
        }

        MaxRetriesExceededSnafu.fail()
    }

    /// The api root. For exemple `https://musicbrainz.org/ws/2`
    pub fn api_root(&self) -> String {
        format!("{}/ws/2", self.musicbrainz_domain)
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum SendWithRetriesError {
    #[snafu(display("Couldn't successfully send the http request with retries"))]
    RequestSendError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: RequestSendError,
    },

    #[snafu(display("The max retry count for the request as been exeeded. You may want to check if the correct url is set, musicbrainz is online, or you aren't hitting the ratelimit."))]
    MaxRetriesExceeded {
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}
