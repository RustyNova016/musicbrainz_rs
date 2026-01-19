use musicbrainz_rs::entity::area::*;
use musicbrainz_rs::prelude::*;

// TODO: find non empty result
#[tokio::test]
#[serial_test::serial]
async fn browse_area_by_collection() {
    let areas = Area::browse()
        .by_collection("6b6dc74a-f779-491a-a0eb-7c1d1ed56fe0")
        .execute_async()
        .await;

    assert!(areas.is_ok());
}
