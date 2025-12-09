use musicbrainz_rs::entity::recording::*;
use musicbrainz_rs::prelude::*;

// use crate::test_framework::check_fetch_query;

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_artists() {
    let association_de_gens_normal = Recording::fetch()
        .id("f5f10cee-5d84-41d0-805d-3503872c151d")
        .with_artists()
        .execute_async()
        .await;

    let artist_credit = association_de_gens_normal.unwrap().artist_credit.unwrap();

    assert!(artist_credit.iter().any(|credit| credit.name == "TTC"));
    assert!(artist_credit.iter().any(|credit| credit.name == "Svinkels"));

    // let association_de_gens_normal = Recording::fetch()
    //     .id("f5f10cee-5d84-41d0-805d-3503872c151d")
    //     .with_artists()
    //     .execute()
    //     .await
    //     .unwrap();

    // check_fetch_query(
    //     "recording/f5f10cee-5d84-41d0-805d-3503872c151d?inc=artist-credits",
    //     association_de_gens_normal,
    //     |rec| assert!(!rec.artist_credit.unwrap().is_empty()),
    // )
    // .await;
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_releases() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_releases()
        .execute_async()
        .await;

    let releases = you_talk_too_much.unwrap().releases;

    assert!(releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Hooker ’n Heat"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_medias() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_releases()
        .with_medias()
        .execute_async()
        .await;

    let releases = you_talk_too_much.unwrap().releases;

    assert!(releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Hooker ’n Heat"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_aliases() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_aliases()
        .execute_async()
        .await;

    let aliases = you_talk_too_much.unwrap().aliases;

    assert!(aliases.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_tags() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_tags()
        .execute_async()
        .await
        .unwrap();

    assert!(you_talk_too_much.tags.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_rating() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_ratings()
        .execute_async()
        .await
        .unwrap();

    assert!(you_talk_too_much.rating.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_genres() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_genres()
        .execute_async()
        .await
        .unwrap();

    assert!(you_talk_too_much.genres.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_annotation() {
    let isolina = Recording::fetch()
        .id("2edf7653-2287-4408-8e7a-20e001a60847")
        .with_annotations()
        .execute_async()
        .await
        .unwrap();

    assert!(isolina.annotation.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_isrcs() {
    let senorita = Recording::fetch()
        .id("62f09fd2-144a-439a-96f9-ce93f05b48ae")
        .with_isrcs()
        .execute_async()
        .await
        .unwrap();

    assert!(senorita.isrcs.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_url_relations() {
    let senorita = Recording::fetch()
        .id("62f09fd2-144a-439a-96f9-ce93f05b48ae")
        .with_url_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = senorita.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "free streaming")); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_work_relations() {
    let senorita = Recording::fetch()
        .id("62f09fd2-144a-439a-96f9-ce93f05b48ae")
        .with_work_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = senorita.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "performance")); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_recording_level_relations() {
    let polly = Recording::fetch()
        .id("af40d6b8-58e8-4ca5-9db8-d4fca0b899e2")
        .with_work_relations()
        .with_work_level_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = polly.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.target_type.as_ref().unwrap() == "work"));
}
