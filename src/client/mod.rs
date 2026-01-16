use std::sync::LazyLock;

use api_bindium::ApiClient;
use api_bindium::ureq::Agent;
use api_bindium::ureq::config::Config;

pub(crate) const DEFAULT_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub static MUSICBRAINZ_CLIENT: LazyLock<MusicBrainzClient> =
    LazyLock::new(MusicBrainzClient::default);

/// Api client for the MB api
///
/// This struct holds the configuration specific to the MB api. To configure the API fetching part, please see [api_bindium::ApiClient]
///
/// # Rate limit
/// By default, there is 5 "Cells", and replenish 1 per second in accordance to the MB API guidelines.
///
/// This allows "bursts" of 5 requests before limiting yourself to the API's classic rate.
/// So you may keep it in mind when designing your apps that you have 5 "free" requests
#[derive(Debug, Clone, bon::Builder)]
pub struct MusicBrainzClient {
    /// The inner API client.
    #[builder(default = MusicBrainzClient::default_api_client())]
    pub api_client: ApiClient,

    /// Domain of the api
    #[builder(default = "musicbrainz.org".to_string())]
    pub musicbrainz_domain: String,

    /// Domain of the cover art archive api
    #[builder(default = "http://coverartarchive.org".to_string())]
    pub coverart_archive_url: String,
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
    /// let client = MusicBrainzClient::new("MyApp/1.0.0 (http://myapp.example.com)").unwrap();
    /// ```
    pub fn new(user_agent: &str) -> Self {
        let agent_conf = Config::builder().user_agent(user_agent).build();
        let agent = Agent::new_with_config(agent_conf);

        Self::builder()
            .api_client(ApiClient::builder().agent(agent).build())
            .build()
    }

    pub fn default_api_client() -> ApiClient {
        let agent_conf = Config::builder().user_agent(DEFAULT_USER_AGENT).build();
        let agent = Agent::new_with_config(agent_conf);

        ApiClient::builder().agent(agent).build()
    }
}

impl Default for MusicBrainzClient {
    fn default() -> Self {
        Self::builder().build()
    }
}
