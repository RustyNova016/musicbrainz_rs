use musicbrainz_rs::Search;
use musicbrainz_rs::entity::artist::*;

#[tokio::test]
#[serial_test::serial]
async fn should_search_artist() {
    let query = ArtistSearchQuery::query_builder()
        .artist("Nirvana")
        .and()
        .artist_type("Group")
        .build();

    let result = Artist::search(query).execute_async().await.unwrap();

    assert!(!result.entities.is_empty());
}
