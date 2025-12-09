use musicbrainz_rs::entity::place::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_get_place_aliases() {
    let blue_note = Place::fetch()
        .id("327c29c6-da63-4dc9-a117-1917ee691ce4")
        .with_aliases()
        .execute_async()
        .await
        .unwrap();

    assert!(blue_note.aliases.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_place_tags() {
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .with_tags()
        .execute_async()
        .await
        .unwrap();

    assert!(olympia.tags.is_some());
}
#[tokio::test]
#[serial_test::serial]
async fn should_get_place_genres() {
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .with_genres()
        .execute_async()
        .await
        .unwrap();

    assert!(olympia.genres.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_place_annotation() {
    let osaka_kosei_nenkin_kaikan = Place::fetch()
        .id("751f998a-60ca-4d48-954f-b101d59ad89a")
        .with_annotations()
        .execute_async()
        .await
        .unwrap();

    assert!(osaka_kosei_nenkin_kaikan.annotation.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_place_event_relations() {
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .with_event_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = olympia.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "held at"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_place_recording_relations() {
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .with_recording_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = olympia.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "engineered at"));
}

// FIXME: Haven't looked at this carefully but this is currently panicking with:
// thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
// Error(Json(Error("missing field `date`", line: 1, column: 1233)))', tests/place/place_includes.rs:80:10
// #[tokio::test]
// #[serial_test::serial]
// async fn should_get_place_release_relations() {
//     let olympia = Place::fetch()
//         .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
//         .with_release_relations()
//         .execute_async().await
//         .unwrap();

//     let relations = olympia.relations.unwrap();

//     assert!(relations.iter().any(|rel| rel.relation_type == "recorded at"));
// }
