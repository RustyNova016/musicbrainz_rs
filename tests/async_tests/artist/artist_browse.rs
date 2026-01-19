use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_browse_artist_by_release_groups() {
    let artistss_on_in_rainbows_rg = Artist::browse()
        .by_release_group("6e335887-60ba-38f0-95af-fae7774336bf")
        .execute_async()
        .await;

    assert!(artistss_on_in_rainbows_rg.is_ok());

    let artistss_on_in_rainbows_rg = artistss_on_in_rainbows_rg.unwrap();

    assert_eq!(artistss_on_in_rainbows_rg.count, 1);
    assert_eq!(artistss_on_in_rainbows_rg.offset, 0);
    assert!(!artistss_on_in_rainbows_rg.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_artist_by_release() {
    let artists_on_in_utero_release = Artist::browse()
        .by_release("18d4e9b4-9247-4b44-914a-8ddec3502103")
        .execute_async()
        .await;

    assert!(artists_on_in_utero_release.is_ok());

    let artists_on_in_utero_release = artists_on_in_utero_release.unwrap();

    assert_eq!(artists_on_in_utero_release.count, 1);
    assert_eq!(artists_on_in_utero_release.offset, 0);
    assert!(!artists_on_in_utero_release.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_artist_by_area() {
    let artistss_in_aberdeen_area = Artist::browse()
        .by_area("a640b45c-c173-49b1-8030-973603e895b5")
        .execute_async()
        .await;

    assert!(artistss_in_aberdeen_area.is_ok());

    let artistss_in_aberdeen_area = artistss_in_aberdeen_area.unwrap();

    assert!(artistss_in_aberdeen_area.count > 0);
    assert_eq!(artistss_in_aberdeen_area.offset, 0);
    assert!(!artistss_in_aberdeen_area.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_artist_by_work() {
    let artists_on_hotel_california = Artist::browse()
        .by_work("22457dc0-ecbf-38f5-9056-11c858530a50")
        .execute_async()
        .await;

    let artists_on_hotel_california = artists_on_hotel_california.unwrap();

    assert!(artists_on_hotel_california.count > 1);
    assert_eq!(artists_on_hotel_california.offset, 0);
    assert!(!artists_on_hotel_california.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_artist_by_recording() {
    let artists_on_polly = Artist::browse()
        .by_recording("af40d6b8-58e8-4ca5-9db8-d4fca0b899e2")
        .execute_async()
        .await;

    let artists_on_polly = artists_on_polly.unwrap();

    assert!(artists_on_polly.count == 1);
    assert_eq!(artists_on_polly.offset, 0);
    assert!(!artists_on_polly.entities.is_empty());
}
