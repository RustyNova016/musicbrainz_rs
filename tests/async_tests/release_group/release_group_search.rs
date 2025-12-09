use musicbrainz_rs::entity::release_group::*;
use musicbrainz_rs::Search;

#[tokio::test]
#[serial_test::serial]
async fn should_search_artist() {
    let query = ReleaseGroupSearchQuery::query_builder()
        .release("Tonight")
        .build();

    let result = ReleaseGroup::search(query).execute_async().await.unwrap();

    assert!(!result.entities.is_empty());
}
