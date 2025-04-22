use std::io;

use musicbrainz_rs::client::MUSICBRAINZ_CLIENT;
use musicbrainz_rs::extra_endpoints::oauth::MusicbrainzOauth;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Please login at: https://musicbrainz.org/oauth2/authorize?response_type=code&client_id=9Q3CwYO0mompOfZqRW6LkggD5xk-qzH5&redirect_uri=urn:ietf:wg:oauth:2.0:oob&scope=submit_isrc");
    // Get the authorization code from the command line
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let oauth = MusicbrainzOauth {
        client_id: "9Q3CwYO0mompOfZqRW6LkggD5xk-qzH5".to_string(),
        client_secret: "nUaOQou6zAX7X8kW8wuuJcGV2efFHKJG".to_string(),
        redirect_uri: "urn:ietf:wg:oauth:2.0:oob".to_string(),
    };

    let mut token = oauth
        .get_access_token(&MUSICBRAINZ_CLIENT, &input)
        .await
        .unwrap();

    // Get the token
    let token_string = token
        .get_or_refresh_token(&MUSICBRAINZ_CLIENT, &oauth)
        .await
        .unwrap();

    println!("Access token: {:?}", token_string);
}
