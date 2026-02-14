use musicbrainz_rs::client::MusicBrainzClient;
use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::prelude::*;

#[macro_rules_attribute::apply(smol_macros::main!)]
async fn main() {
    let client = MusicBrainzClient::builder()
        .musicbrainz_domain("musicbrainz.org".to_string())
        .netrc_auth()
        .build();

    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute_with_client(&client);

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
