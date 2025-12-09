use musicbrainz_rs::entity::area::AreaType::*;
use musicbrainz_rs::entity::area::*;
use musicbrainz_rs::Search;

#[tokio::test]
#[serial_test::serial]
async fn should_search_area() {
    let query = AreaSearchQuery::query_builder()
        .area("London")
        .and()
        .tag("place")
        .build();

    let result = Area::search(query).execute_async().await.unwrap();

    assert!(result
        .entities
        .iter()
        .any(|area| area.area_type.as_ref().unwrap() == &City));
}
