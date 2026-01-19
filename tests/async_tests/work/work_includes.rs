use musicbrainz_rs::entity::work::Work;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_tags() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_tags()
        .execute_async()
        .await
        .unwrap();

    assert!(
        hotel_california
            .tags
            .unwrap()
            .iter()
            .any(|tag| tag.name == "ripped off")
    );
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_aliases() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_aliases()
        .execute_async()
        .await
        .unwrap();

    assert!(hotel_california.aliases.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_rating() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_ratings()
        .execute_async()
        .await
        .unwrap();

    assert!(hotel_california.rating.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_genres() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_genres()
        .execute_async()
        .await
        .unwrap();

    assert!(hotel_california.genres.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_annotation() {
    let vater_unser_im_himmelreich = Work::fetch()
        .id("85ab2b66-cf0b-47e9-beee-34c64a5ddea1")
        .with_annotations()
        .execute_async()
        .await
        .unwrap();

    assert!(vater_unser_im_himmelreich.annotation.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_artist_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_artist_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "writer"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_label_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_label_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(
        relations
            .iter()
            .any(|rel| rel.relation_type == "publishing")
    );
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_recording_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_recording_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(
        relations
            .iter()
            .any(|rel| rel.relation_type == "performance")
    );
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_url_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_url_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "wikidata"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_work_work_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_work_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "based on"));
}
