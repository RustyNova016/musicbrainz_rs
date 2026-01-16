use musicbrainz_rs::Search;
use musicbrainz_rs::entity::work::*;

#[tokio::test]
#[serial_test::serial]
async fn should_search_work() {
    let query = WorkSearchQuery::query_builder()
        .work("vater unser im")
        .and()
        .arid("ea547ae6-8ab1-48b2-b1a9-70a638d2ad26")
        .build();

    let result = Work::search(query).execute_async().await.unwrap();

    assert!(
        result
            .entities
            .iter()
            .any(|work| work.title == "Vater unser im Himmelreich")
    );
}
