use serde::de::DeserializeOwned;

use crate::client::MusicBrainzClient;
use crate::entity::api::MusicbrainzResult;

impl MusicBrainzClient {
    /// Send the reqwest as a get, deal with ratelimits, and retries
    #[maybe_async::maybe_async]
    pub(crate) async fn get<T>(&self, url: &str) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
    {
        self.send_with_retries(|| self.reqwest_client.get(url).send())
            .await?
            .json::<MusicbrainzResult<T>>()
            .await?
            .into_result(url.to_string())
    }
}
