use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::entity::recording::RecordingSearchQuery;
use musicbrainz_rs::Search;

#[tokio::test]
#[serial_test::serial]
async fn should_paginate_search() {
    let query = RecordingSearchQuery::query_builder()
        .recording("love")
        .build();

    let result = Recording::search(query.clone())
        .offset(0)
        .limit(10)
        .execute_async()
        .await
        .unwrap();
    assert!(result.count > 0);
    assert!(result.entities.len() == 10);

    let result_next = Recording::search(query)
        .offset(10)
        .limit(10)
        .execute_async()
        .await
        .unwrap();
    assert!(result_next.count > 0);
    assert!(result_next.entities.len() == 10);
    assert!(result.entities.first() != result_next.entities.first())
}
