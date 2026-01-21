use api_bindium::endpoints::EndpointUriBuilder;
use api_bindium::endpoints::path::EndpointUriBuilderPath;

/// Endpoints for the api
#[derive(Debug, bon::Builder, Clone)]
pub struct MusicBrainzAPIEnpoints {
    /// The domain of the server.
    ///
    /// Please note that all the api endpoints must be accessed by HTTPS.
    #[builder(default = "musicbrainz.org".to_string())]
    domain: String,
}

impl MusicBrainzAPIEnpoints {
    /// The api root
    pub fn api_root(&self) -> String {
        format!("https://{}", self.domain)
    }

    /// Return an endpoint builder for endpoints
    ///
    /// The scheme and domain are already set
    pub fn endpoint_builder(&self) -> EndpointUriBuilder<EndpointUriBuilderPath> {
        EndpointUriBuilder::new()
            .https()
            .set_authority(&self.domain)
    }
}

impl Default for MusicBrainzAPIEnpoints {
    fn default() -> Self {
        Self::builder().build()
    }
}
