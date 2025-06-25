use core::time::Duration;
use std::sync::LazyLock;

use reqwest::header;
use reqwest::header::InvalidHeaderValue;
use serde::de::DeserializeOwned;

#[cfg(feature = "blocking")]
use std::thread::sleep;

#[cfg(feature = "async")]
use tokio::time::sleep;

#[cfg(feature = "rate_limit")]
use core::num::NonZeroU32;
#[cfg(feature = "rate_limit")]
use governor::{
    clock, middleware::NoOpMiddleware, state::InMemoryState, state::NotKeyed, Quota, RateLimiter,
};
#[cfg(feature = "rate_limit")]
use std::sync::Arc;

use crate::entity::api::MusicbrainzResult;
use crate::reqwester::RequestBuilder;
use crate::reqwester::ReqwestClient;
use crate::reqwester::Response;
use crate::BASE_COVERART_URL;
use crate::BASE_URL;
use crate::DEFAULT_USER_AGENT;
use crate::HTTP_RATELIMIT_CODE;

pub static MUSICBRAINZ_CLIENT: LazyLock<MusicBrainzClient> =
    LazyLock::new(MusicBrainzClient::default);

#[derive(Debug, Clone)]
pub struct MusicBrainzClient {
    /// Domain of the api. Aka, `https://musicbrainz.org`
    pub musicbrainz_domain: String,
    pub coverart_archive_url: String,
    pub(crate) user_agent: String,
    pub max_retries: u32,

    pub(crate) reqwest_client: ReqwestClient,

    /// The rate limiter of the API. By default, it has 5 "Cells", and replenish 1 per second in accordance to the MB API guidelines.
    ///
    /// This allows "bursts" of 5 requests before limiting yourself to the API's classic rate.
    /// So you may keep it in mind when designing your apps that you have 5 "free" requests
    #[cfg(feature = "rate_limit")]
    pub rate_limit:
        Option<Arc<RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>>>,
}

// Common implements
impl MusicBrainzClient {
    /// Each request sent to MusicBrainz needs to include a User-Agent header,
    /// with enough information in the User-Agent to contact the application maintainers.
    /// We strongly suggest including your application's version number
    /// in the User-Agent string too.
    ///
    /// For more info see [Rate Limiting](https://musicbrainz.org/doc/MusicBrainz_API/Rate_Limiting#Provide_meaningful_User-Agent_strings)
    ///
    /// ## Example
    /// ```rust
    /// # use musicbrainz_rs::client::MusicBrainzClient;
    /// # let mut client = MusicBrainzClient::default();
    /// client.set_user_agent("MyAwesomeTagger/1.2.0 ( http://myawesometagger.example.com )");
    /// ```
    pub fn set_user_agent(&mut self, user_agent: &str) -> Result<(), InvalidHeaderValue> {
        self.user_agent = user_agent.to_string();

        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&self.user_agent)?,
        );

        self.reqwest_client = ReqwestClient::builder()
            // see : https://github.com/hyperium/hyper/issues/2136
            .pool_max_idle_per_host(0)
            .default_headers(headers)
            .build().expect("Unable to set default user agent, the following values must be set in Cargo.toml : 'name', 'version', 'authors'");

        Ok(())
    }

    /// Remove the rate limiter
    #[cfg(feature = "rate_limit")]
    pub fn drop_ratelimit(&mut self) {
        self.rate_limit = None;
    }

    /// Return the reqwest client to allow custom queries
    pub fn get_reqwest_client(&self) -> &ReqwestClient {
        &self.reqwest_client
    }
}

impl MusicBrainzClient {
    #[maybe_async::maybe_async]
    pub async fn wait_for_ratelimit(&self) {
        #[cfg(feature = "rate_limit")]
        if let Some(val) = &self.rate_limit {
            val.until_ready().await
        }
    }

    /// Send the reqwest as a get, deal with ratelimits, and retries
    #[maybe_async::maybe_async]
    pub(crate) async fn get<T>(&self, url: &str) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
    {
        self.send_with_retries(self.reqwest_client.get(url))
            .await?
            .json::<MusicbrainzResult<T>>()
            .await?
            .into_result(url.to_string())
    }

    /// Send the reqwest, deal with ratelimits, and retries
    #[maybe_async::maybe_async]
    pub(crate) async fn send_with_retries(
        &self,
        request: RequestBuilder,
    ) -> Result<Response, crate::Error> {
        let mut retries = 0;

        while retries != self.max_retries {
            self.wait_for_ratelimit().await;

            // Send the query
            let request = request.try_clone().unwrap();
            let response = request.send().await?;

            // Let's check if we hit the rate limit
            if response.status().as_u16() == HTTP_RATELIMIT_CODE {
                // Oh no. Let's wait the timeout
                let headers = response.headers();
                let retry_secs = headers.get("retry-after").unwrap().to_str().unwrap();
                let duration = Duration::from_secs(retry_secs.parse::<u64>().unwrap() + 1);
                sleep(duration).await;
                retries += 1;

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

        Err(crate::Error::MaxRetriesExceeded)
    }

    /// The api root. For exemple `https://musicbrainz.org/ws/2`
    pub fn api_root(&self) -> String {
        format!("{}/ws/2", self.musicbrainz_domain)
    }
}

impl Default for MusicBrainzClient {
    fn default() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(DEFAULT_USER_AGENT),
        );

        let reqwest_client = ReqwestClient::builder()
            // see : https://github.com/hyperium/hyper/issues/2136
            .pool_max_idle_per_host(0)
            .default_headers(headers)
            .build().expect("Unable to set default user agent, the following values must be set in Cargo.toml : 'name', 'version', 'authors'");

        #[cfg(feature = "rate_limit")]
        let quota =
            Quota::per_second(NonZeroU32::new(1).unwrap()).allow_burst(NonZeroU32::new(5).unwrap());

        Self {
            musicbrainz_domain: BASE_URL.to_string(),
            coverart_archive_url: BASE_COVERART_URL.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_owned(),
            max_retries: 10,

            reqwest_client,
            #[cfg(feature = "rate_limit")]
            rate_limit: Some(Arc::new(RateLimiter::direct(quota))),
        }
    }
}

// #[cfg(test)]
// #[cfg(feature = "rate_limit")]
// mod tests {
//     use futures::stream;
//     use futures::StreamExt;

//     use crate::entity::recording::Recording;
//     use crate::Fetch;

//     #[tokio::test]
//     #[serial_test::serial]

//     async fn should_not_hit_ratelimit() {
//         stream::iter(0..30)
//             .map(|_| async move {
//                 Recording::fetch()
//                     .id("5fed738b-1e5c-4a1b-9f66-b3fd15dbc8ef")
//                     .execute()
//                     .await
//             })
//             .buffer_unordered(20)
//             .collect::<Vec<_>>()
//             .await;
//     }
// }
