use musicbrainz_rs::client::MusicBrainzClient;
use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn set_user_agent_should_work() {
    let client = MusicBrainzClient::new("musicbrainz_rs_testing/0.9").unwrap();

    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute_with_client(&client)
        .await;

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
