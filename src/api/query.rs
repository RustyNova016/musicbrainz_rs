use core::marker::PhantomData;

use api_bindium::endpoints::EndpointUriBuilder;
use api_bindium::endpoints::query::EndpointUriBuilderQuery;

use crate::client::MusicBrainzClient;
use crate::entity::Include;

/// The base element of a query
#[derive(Clone, Debug)]
pub struct Query<T> {
    /// The path of the api to query
    pub(crate) path: String,

    /// The includes added to the query
    pub(crate) include: Vec<Include>,

    /// The resulting type of the query
    pub(crate) result_type: PhantomData<T>,
}

impl<T> Query<T> {
    /// Add an include parameter to the query
    pub(crate) fn include(&mut self, include: Include) -> &mut Self {
        self.include.push(include);
        self
    }

    /// Create the full url path of the query
    pub(crate) fn get_endpoint(
        &self,
        client: &MusicBrainzClient,
    ) -> EndpointUriBuilder<EndpointUriBuilderQuery> {
        let url = EndpointUriBuilder::new()
            .http()
            .set_authority(&client.musicbrainz_domain)
            .add_path_fragment("ws/2")
            .add_path_fragment(&self.path)
            .query()
            .add_parameter("fmt", "json");

        // If we don't have includes, let's return early
        if self.include.is_empty() {
            return url;
        }

        let mut incl = String::new();

        for inc in &self.include {
            incl.push_str(inc.as_str());
            if Some(inc) != self.include.last() {
                incl.push('+');
            }
        }

        url.add_parameter("inc", incl)
    }
}
