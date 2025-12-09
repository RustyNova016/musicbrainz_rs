use musicbrainz_rs::entity::event::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_browse_event_by_place() {
    let events_in_north_stage_woodstock_1994 = Event::browse()
        .by_place("380bad3f-d3d7-4a1c-9e7f-c6ec2661165c")
        .execute_async()
        .await;

    assert!(events_in_north_stage_woodstock_1994.is_ok());

    let events_in_north_stage_woodstock_1994 = events_in_north_stage_woodstock_1994.unwrap();

    assert!(events_in_north_stage_woodstock_1994.count > 1);
    assert_eq!(events_in_north_stage_woodstock_1994.offset, 0);
    assert!(!events_in_north_stage_woodstock_1994.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_event_by_artist() {
    let events_with_aerosmith = Event::browse()
        .by_artist("3d2b98e5-556f-4451-a3ff-c50ea18d57cb")
        .execute_async()
        .await;

    assert!(events_with_aerosmith.is_ok());

    let events_with_aerosmith = events_with_aerosmith.unwrap();

    assert!(events_with_aerosmith.count > 1);
    assert_eq!(events_with_aerosmith.offset, 0);
    assert!(!events_with_aerosmith.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_event_by_area() {
    let events_in_montreux = Event::browse()
        .by_area("d872ed01-edfd-4b39-8ab5-f8b3c84fc001")
        .execute_async()
        .await;

    assert!(events_in_montreux.is_ok());

    let events_in_montreux = events_in_montreux.unwrap();

    assert!(events_in_montreux.count > 1);
    assert_eq!(events_in_montreux.offset, 0);
    assert!(!events_in_montreux.entities.is_empty());
}
