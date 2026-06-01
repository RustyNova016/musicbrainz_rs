use api_bindium::endpoints::EndpointUriBuilder;
use api_bindium::endpoints::path::EndpointUriBuilderPath;

/// Endpoints for the api
#[derive(Debug, bon::Builder, Clone)]
pub struct MusicBrainzAPIEnpoints {
    /// Whether to use HTTPS. Defaults to `true`.
    #[builder(default = true)]
    use_https: bool,

    /// The authority (host and optional port) of the server,
    /// e.g. `musicbrainz.org` or `localhost:5000`.
    #[builder(default = "musicbrainz.org".to_string())]
    authority: String,
}

impl MusicBrainzAPIEnpoints {
    /// The api root URL
    pub fn api_root(&self) -> String {
        format!(
            "{}://{}",
            if self.use_https { "https" } else { "http" },
            self.authority,
        )
    }

    /// Return an endpoint builder with the scheme and authority already set
    pub fn endpoint_builder(&self) -> EndpointUriBuilder<EndpointUriBuilderPath> {
        let builder = EndpointUriBuilder::new();
        if self.use_https {
            builder.https()
        } else {
            builder.http()
        }
        .set_authority(&self.authority)
    }
}

impl Default for MusicBrainzAPIEnpoints {
    fn default() -> Self {
        Self::builder().build()
    }
}
