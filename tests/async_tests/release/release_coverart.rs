use musicbrainz_rs::Fetch;
use musicbrainz_rs::FetchCoverart;
use musicbrainz_rs::entity::CoverartResponse;
use musicbrainz_rs::entity::release::*;

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_coverart() {
    let in_utero_coverart = Release::fetch_coverart()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute_async()
        .await
        .expect("Unable to get cover art");

    if let CoverartResponse::Json(coverart) = in_utero_coverart {
        assert!(coverart.images[0].front);
        assert!(!coverart.images[0].back);
    } else {
        panic!();
    }
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_coverart_after_fetch() {
    let in_utero = Release::fetch()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute_async()
        .await
        .expect("Unable to get release");

    let in_utero_coverart = in_utero
        .get_coverart()
        .execute_async()
        .await
        .expect("Unable to get coverart");

    if let CoverartResponse::Json(coverart) = in_utero_coverart {
        assert!(coverart.images[0].front);
        assert!(!coverart.images[0].back);
    } else {
        panic!();
    }
}
