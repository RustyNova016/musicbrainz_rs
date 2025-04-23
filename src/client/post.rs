use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;

use crate::client::MusicBrainzClient;
use crate::entity::api::MusicbrainzResult;

impl MusicBrainzClient {
    #[maybe_async::maybe_async]
    pub async fn post<T>(&self, token: &str, path: &str, body: &str) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!(
            "http://{}/{path}?client={}",
            self.musicbrainz_domain, self.user_agent
        );

        self.send_with_retries(|| {
            self.reqwest_client
                .post(url.to_string())
                .body(body.to_string())
                .header(AUTHORIZATION, format!("Bearer {token}"))
                .send()
        })
        .await?
        .json::<MusicbrainzResult<T>>()
        .await?
        .into_result(url.to_string())
    }
}
