use musicbrainz_rs::entity::area::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_tags() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_tags()
        .execute_async()
        .await
        .unwrap();

    assert!(aberdeen.tags.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_aliases() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_aliases()
        .execute_async()
        .await
        .unwrap();

    assert!(aberdeen.aliases.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_genres() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_genres()
        .execute_async()
        .await
        .unwrap();

    assert!(aberdeen.genres.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_annotation() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_annotations()
        .execute_async()
        .await
        .unwrap();

    assert!(london.annotation.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_area_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_area_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "part of"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_event_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_event_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "held in"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_recording_relations() {
    let ontario = Area::fetch()
        .id("2747553f-b44d-44c4-a7c3-b67412b6f10b")
        .with_recording_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = ontario.relations.unwrap();

    assert!(
        relations
            .iter()
            .any(|rel| rel.relation_type == "recorded in")
    );
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_release_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_release_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(
        relations
            .iter()
            .any(|rel| rel.relation_type == "engineered in")
    );
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_url_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_url_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "geonames"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_area_work_relations() {
    let andorra = Area::fetch()
        .id("e01da61e-99a8-3c76-a27d-774c3f4982f0")
        .with_work_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = andorra.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "anthem"));
}
