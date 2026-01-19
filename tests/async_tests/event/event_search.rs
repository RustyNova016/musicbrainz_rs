use musicbrainz_rs::Search as _;
use musicbrainz_rs::entity::event::*;

#[tokio::test]
#[serial_test::serial]
async fn should_search_event() {
    let query = EventSearchQuery::query_builder()
        .event("kiss at huntington center")
        .and()
        .arid("e935d070-004d-405a-8b9d-1d9e51590b55")
        .build();

    let result = Event::search(query).execute_async().await.unwrap();

    assert!(
        result
            .entities
            .iter()
            .any(|event| event.name == "KISS at Huntington Center")
    );
}
