use musicbrainz_rs::Search;
use musicbrainz_rs::entity::release::*;

#[tokio::test]
#[serial_test::serial]
async fn should_search_artist() {
    let query = ReleaseSearchQuery::query_builder()
        .release("Drivers License")
        .build();

    let result = Release::search(query).execute_async().await.unwrap();

    assert!(
        result
            .entities
            .iter()
            .any(|release| release.title == "drivers license")
    );
}
