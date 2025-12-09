use musicbrainz_rs::entity::label::Label;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_releases() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_releases()
        .execute_async()
        .await;

    let releases = ninja_tune.unwrap().releases;

    assert!(releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Zen Brakes, Volume 1"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_aliases() {
    let motown = Label::fetch()
        .id("8e479e57-ef44-490c-b75d-cd28df89bf1b")
        .with_aliases()
        .execute_async()
        .await;

    let aliases = motown.unwrap().aliases;

    assert!(aliases
        .unwrap()
        .iter()
        .any(|alias| alias.name == "Motown Records"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_tags() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_tags()
        .execute_async()
        .await
        .unwrap();

    assert!(ninja_tune
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "independent"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_rating() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_ratings()
        .execute_async()
        .await
        .unwrap();

    assert!(ninja_tune.rating.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_genres() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_genres()
        .execute_async()
        .await
        .unwrap();

    assert!(ninja_tune.genres.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_annotation() {
    let tokuma_japan_communications = Label::fetch()
        .id("040439f9-578b-45b6-b07b-d6c97e544859")
        .with_annotations()
        .execute_async()
        .await
        .unwrap();

    assert!(tokuma_japan_communications.annotation.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_artist_relations() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_artist_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = ninja_tune.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "label founder"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_label_relations() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_label_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = ninja_tune.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "label distribution"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_recording_relations() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_recording_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = ninja_tune.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "phonographic copyright"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_release_relations() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_release_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = ninja_tune.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "copyright"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_label_url_relations() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_url_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = ninja_tune.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "blog"));
}
