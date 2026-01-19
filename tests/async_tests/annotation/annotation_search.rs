use musicbrainz_rs::Search;
use musicbrainz_rs::entity::annotation::*;

#[tokio::test]
#[serial_test::serial]
async fn should_search_annotation() {
    let query = AnnotationSearchQuery::query_builder()
        .text("Warner Classics International")
        .build();

    let result = Annotation::search(query).execute_async().await.unwrap();

    assert!(
        result
            .entities
            .iter()
            .any(|annotation| annotation.name == "Warner Classics")
    );
}
