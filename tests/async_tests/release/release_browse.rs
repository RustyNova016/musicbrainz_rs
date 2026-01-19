use musicbrainz_rs::entity::release::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_artist() {
    let releases_by_svinkels = Release::browse()
        .by_artist("770d490e-c89b-4775-8508-aca7c75142cd")
        .execute_async()
        .await;

    //assert!(releases_by_svinkels.is_ok());

    let releases_by_svinkels = releases_by_svinkels.unwrap();

    assert!(releases_by_svinkels.count > 1);
    assert_eq!(releases_by_svinkels.offset, 0);
    assert!(!releases_by_svinkels.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_area() {
    let releases_france = Release::browse()
        .by_area("08310658-51eb-3801-80de-5a0739207115")
        .execute_async()
        .await;

    assert!(releases_france.is_ok());

    let releases_france = releases_france.unwrap();

    assert!(releases_france.count > 1);
    assert_eq!(releases_france.offset, 0);
    assert!(!releases_france.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_label() {
    let ninjatune_releases = Release::browse()
        .by_label("dc940013-b8a8-4362-a465-291026c04b42")
        .execute_async()
        .await;

    assert!(ninjatune_releases.is_ok());

    let ninjatune_releases = ninjatune_releases.unwrap();

    assert!(ninjatune_releases.count > 1);
    assert_eq!(ninjatune_releases.offset, 0);
    assert!(!ninjatune_releases.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_recording() {
    let release_of_l_ecole_du_micro_d_argent = Release::browse()
        .by_recording("72984ccf-9edb-44d4-aad2-f41c9ef5414a")
        .execute_async()
        .await;

    assert!(release_of_l_ecole_du_micro_d_argent.is_ok());

    let release_of_l_ecole_du_micro_d_argent = release_of_l_ecole_du_micro_d_argent.unwrap();

    assert!(release_of_l_ecole_du_micro_d_argent.count > 1);
    assert_eq!(release_of_l_ecole_du_micro_d_argent.offset, 0);
    assert!(!release_of_l_ecole_du_micro_d_argent.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_track() {
    let release_with_phantom_by_justice = Release::browse()
        .by_track("d5bbc037-eace-3712-9af8-ecb378b65dd6")
        .execute_async()
        .await;

    assert!(release_with_phantom_by_justice.is_ok());

    let release_with_phantom_by_justice = release_with_phantom_by_justice.unwrap();

    assert!(release_with_phantom_by_justice.count > 0);
    assert_eq!(release_with_phantom_by_justice.offset, 0);
    assert!(!release_with_phantom_by_justice.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_track_artist() {
    let release_featuring_akhenaton = Release::browse()
        .by_track_artist("55808e24-5150-4abd-b86b-7c3d7a080da3")
        .execute_async()
        .await;

    assert!(release_featuring_akhenaton.is_ok());

    let release_featuring_akhenaton = release_featuring_akhenaton.unwrap();

    assert!(release_featuring_akhenaton.count > 1);
    assert_eq!(release_featuring_akhenaton.offset, 0);
    assert!(!release_featuring_akhenaton.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_track_artist_with_recordings() {
    let release_featuring_akhenaton = Release::browse()
        .by_track_artist("55808e24-5150-4abd-b86b-7c3d7a080da3")
        .with_recordings()
        .execute_async()
        .await;

    let release_featuring_akhenaton = release_featuring_akhenaton.unwrap();

    let release = &release_featuring_akhenaton.entities[0];
    let medias = release.media.as_ref().unwrap();
    assert!(!medias.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_release_group() {
    let neil_young_harvest_releases = Release::browse()
        .by_release_group("b25419cf-71bf-3a54-8cd4-2161c61056a0")
        .execute_async()
        .await;

    assert!(neil_young_harvest_releases.is_ok());

    let neil_young_harvest_releases = neil_young_harvest_releases.unwrap();

    assert!(neil_young_harvest_releases.count > 1);
    assert_eq!(neil_young_harvest_releases.offset, 0);
    assert!(!neil_young_harvest_releases.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_by_collection() {
    let releases_in_collection = Release::browse()
        .by_collection("91565a03-bce8-47e9-ab70-e4d4e1684d7f")
        .execute_async()
        .await;

    assert!(releases_in_collection.is_ok());

    let releases_in_collection = releases_in_collection.unwrap();

    assert!(releases_in_collection.count > 1);
    assert_eq!(releases_in_collection.offset, 0);
    assert!(!releases_in_collection.entities.is_empty());
}
