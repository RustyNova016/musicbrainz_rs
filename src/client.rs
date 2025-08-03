use std::sync::LazyLock;

use reqwest::header;
use reqwest::header::InvalidHeaderValue;

#[cfg(feature = "rate_limit")]
use core::num::NonZeroU32;
#[cfg(feature = "rate_limit")]
use governor::{
    clock, middleware::NoOpMiddleware, state::InMemoryState, state::NotKeyed, Quota, RateLimiter,
};
#[cfg(feature = "rate_limit")]
use std::sync::Arc;

use crate::config::BASE_COVERART_URL;
use crate::config::BASE_URL;
use crate::config::DEFAULT_USER_AGENT;
use crate::reqwester::ReqwestClient;

pub static MUSICBRAINZ_CLIENT: LazyLock<MusicBrainzClient> =
    LazyLock::new(MusicBrainzClient::default);

#[derive(Debug, Clone)]
pub struct MusicBrainzClient {
    /// Domain of the api. Aka, `https://musicbrainz.org`
    pub musicbrainz_domain: String,
    pub coverart_archive_url: String,
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
    /// Creates a new [MusicBrainzClient] with the specified user agent.
    ///
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
    /// let client = MusicBrainzClient::new("MyApp/1.0.0 (http://myapp.example.com)");
    /// ```
    pub fn new(user_agent: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(user_agent).expect("Unable to set default user agent"),
        );

        let reqwest_client = ReqwestClient::builder()
            // see : https://github.com/hyperium/hyper/issues/2136
            .pool_max_idle_per_host(0)
            .default_headers(headers)
            .build().expect("Unable to set default user agent, the following values must be set in Cargo.toml : 'name', 'version', 'authors'");

        Self {
            reqwest_client,
            ..Default::default()
        }
    }

    /// Creates a new [MusicBrainzClient] using an existing [ReqwestClient].
    ///
    /// ```rust
    /// use musicbrainz_rs::client::MusicBrainzClient;
    /// use reqwest::Client as ReqwestClient;
    /// let reqwest_client = ReqwestClient::builder().build().unwrap();
    /// let client = MusicBrainzClient::new_with_reqwest_client(reqwest_client);
    /// ```
    pub fn new_with_reqwest_client(reqwest_client: ReqwestClient) -> Self {
        Self {
            reqwest_client,
            ..Default::default()
        }
    }

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
    ///
    /// > WARNING : This method will override the [ReqwestClient]
    #[deprecated(note = "Use `new` instead")]
    pub fn set_user_agent(&mut self, user_agent: &str) -> Result<(), InvalidHeaderValue> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(user_agent)?,
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
