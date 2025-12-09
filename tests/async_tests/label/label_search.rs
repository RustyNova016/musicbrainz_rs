use musicbrainz_rs::entity::label::*;
use musicbrainz_rs::Search;

#[tokio::test]
#[serial_test::serial]
async fn should_search_label() {
    let query = LabelSearchQuery::query_builder()
        .label("Abbey Road Studios")
        .build();

    let result = Label::search(query).execute_async().await.unwrap();

    assert!(result
        .entities
        .iter()
        .any(|label| label.label_type.as_ref().unwrap() == &LabelType::Production));
}
