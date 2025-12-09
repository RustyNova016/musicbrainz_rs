use musicbrainz_rs::entity::event::Event;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_tags() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .with_tags()
        .execute_async()
        .await
        .unwrap();

    assert!(dour_festival_1989.tags.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_aliases() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .with_aliases()
        .execute_async()
        .await
        .unwrap();

    assert!(dour_festival_1989.aliases.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_rating() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .with_ratings()
        .execute_async()
        .await
        .unwrap();

    assert!(dour_festival_1989.rating.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_genres() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .with_genres()
        .execute_async()
        .await
        .unwrap();

    assert!(dour_festival_1989.genres.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_annotation() {
    let kiss_at_huntington_center = Event::fetch()
        .id("24610e7f-eaa3-4c45-9f06-7f441b1a5dd7")
        .with_annotations()
        .execute_async()
        .await
        .unwrap();

    assert!(kiss_at_huntington_center.annotation.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_artist_relations() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .with_artist_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = dour_festival_1989.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "main performer"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_place_relations() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .with_place_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = dour_festival_1989.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "held at"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_series_relations() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .with_series_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = dour_festival_1989.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "part of"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_event_url_relations() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .with_url_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = dour_festival_1989.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "songkick"));
}
