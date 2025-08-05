use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::client::MusicBrainzClient;

/// Oauth data for Musicbrainz.
///
/// You can load it directly from a json/toml file to hide your credentials. For applications you create it in code
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MusicbrainzOauth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl MusicbrainzOauth {
    /// Get an access token from a authorization code
    #[maybe_async::maybe_async]
    pub async fn get_access_token(
        &self,
        client: &MusicBrainzClient,
        auth_code: &str,
    ) -> Result<MusicbrainzToken, crate::Error> {
        Ok(
            client.reqwest_client
                    .post(format!(
                        "https://{}/oauth2/token?grant_type=authorization_code&code={}&client_id={}&client_secret={}&redirect_uri={}",
                        client.musicbrainz_domain, auth_code, self.client_id, self.client_secret, self.redirect_uri
                    ))
                    .send()
                    .await?
                    .json::<TokenResponse>()
                    .await?
                    .into()
                )
    }

    /// Refresh a token
    #[maybe_async::maybe_async]
    pub async fn refresh_token(
        &self,
        client: &MusicBrainzClient,
        refresh_token: &str,
    ) -> Result<TokenResponse, crate::Error> {
        Ok( client.reqwest_client
            .post(format!(
                "https://{}/oauth2/token?grant_type=refresh_token&refresh_token={}&client_id={}&client_secret={}", 
                client.musicbrainz_domain, refresh_token, self.client_id, self.client_secret))
            .send()
            .await?
            .json()
            .await?)
    }
}

/// An user token
///
/// It is recomended to store it somewhere between runs to avoid asking the user to log back in every time.
/// You can use serde to save it to a json or toml file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MusicbrainzToken {
    refresh_token: String,
    token: String,
    token_expire: DateTime<Utc>,
}

impl MusicbrainzToken {
    /// Return true if the token is expired, or missing
    pub fn is_token_expired(&self) -> bool {
        Utc::now() >= self.token_expire
    }

    /// Get the current token or refresh it.
    #[maybe_async::maybe_async]
    pub async fn get_or_refresh_token(
        &mut self,
        client: &MusicBrainzClient,
        oauth: &MusicbrainzOauth,
    ) -> Result<&str, crate::Error> {
        if self.is_token_expired() {
            let token = oauth.refresh_token(client, &self.refresh_token).await?;
            *self = token.into();
        }

        Ok(self.token.as_str())
    }
}

impl From<TokenResponse> for MusicbrainzToken {
    fn from(token: TokenResponse) -> Self {
        MusicbrainzToken {
            refresh_token: token.refresh_token,
            token: token.access_token,
            token_expire: Utc::now() + Duration::seconds(token.expires_in - 60),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
}
