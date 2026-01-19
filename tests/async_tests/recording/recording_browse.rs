use musicbrainz_rs::entity::recording::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_browse_recording_by_artist() {
    let recording_by_svinkels = Recording::browse()
        .by_artist("770d490e-c89b-4775-8508-aca7c75142cd")
        .execute_async()
        .await;

    assert!(recording_by_svinkels.is_ok());

    let recording_by_svinkels = recording_by_svinkels.unwrap();

    assert!(recording_by_svinkels.count > 1);
    assert_eq!(recording_by_svinkels.offset, 0);
    assert!(!recording_by_svinkels.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_recording_work() {
    let la_javanaise_recordings = Recording::browse()
        .by_work("578eab03-84d3-374f-a7c5-03c3a685a9a5")
        .execute_async()
        .await;

    assert!(la_javanaise_recordings.is_ok());

    let la_javanaise_recordings = la_javanaise_recordings.unwrap();

    assert!(la_javanaise_recordings.count > 1);
    assert_eq!(la_javanaise_recordings.offset, 0);
    assert!(!la_javanaise_recordings.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_recording_by_release() {
    let recording_on_hooker_n_heat = Recording::browse()
        .by_release("38860ba5-6b40-3e19-83ae-a560737a3f6f")
        .execute_async()
        .await;

    assert!(recording_on_hooker_n_heat.is_ok());

    let recording_on_hooker_n_heat = recording_on_hooker_n_heat.unwrap();

    assert!(recording_on_hooker_n_heat.count > 1);
    assert_eq!(recording_on_hooker_n_heat.offset, 0);
    assert!(!recording_on_hooker_n_heat.entities.is_empty());
}
