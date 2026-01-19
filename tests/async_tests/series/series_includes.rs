use musicbrainz_rs::entity::series::Series;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_get_series_tags() {
    let breaks_loop_n_edits = Series::fetch()
        .id("0c66e70d-5f23-4579-8fe5-6bc0007428a2")
        .with_tags()
        .execute_async()
        .await
        .unwrap();

    assert!(
        breaks_loop_n_edits
            .tags
            .unwrap()
            .iter()
            .any(|tag| tag.name == "breakbeat")
    );
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_series_aliases() {
    let ultimate_breaks_and_beats = Series::fetch()
        .id("3e5979c8-5a78-4d0b-878a-0fb87853effe")
        .with_aliases()
        .execute_async()
        .await
        .unwrap();

    assert!(ultimate_breaks_and_beats.aliases.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_series_genres() {
    let ultimate_breaks_and_beats = Series::fetch()
        .id("3e5979c8-5a78-4d0b-878a-0fb87853effe")
        .with_genres()
        .execute_async()
        .await
        .unwrap();

    assert!(ultimate_breaks_and_beats.genres.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_series_annotation() {
    let record_store_day_2020 = Series::fetch()
        .id("c1071cec-48f1-4231-ac8e-8c64e15ec7cd")
        .with_annotations()
        .execute_async()
        .await
        .unwrap();

    assert!(record_store_day_2020.annotation.is_some());
}

// FIXME: This is currently panicking with:
// thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
// Error(Json(Error("no variant of enum RelationContent found in flattened data", line: 1, column: 593)))'
// #[tokio::test]
// #[serial_test::serial]
// async fn should_get_series_release_group_relations() {
//     let ultimate_breaks_and_beats = Series::fetch()
//         .id("3e5979c8-5a78-4d0b-878a-0fb87853effe")
//         .with_release_group_relations()
//         .execute_async().await
//         .unwrap();

//     let relations = ultimate_breaks_and_beats.relations.unwrap();

//     assert!(relations.iter().any(|rel| rel.relation_type == "part of"));
// }
