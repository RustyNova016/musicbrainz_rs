use musicbrainz_rs_nova::entity::release_group::*;
use musicbrainz_rs_nova::entity::CoverartResponse;
use musicbrainz_rs_nova::Fetch;
use musicbrainz_rs_nova::FetchCoverart;

#[test]
fn should_get_release_group_coverart() {
    let echoes_coverart = ReleaseGroup::fetch_coverart()
        .id("ccdb3c9b-67e8-46f5-803f-026ef815ceea")
        .execute()
        .expect("Unable to get cover art");

    if let CoverartResponse::Json(coverart) = echoes_coverart {
        assert!(coverart.images[0].front);
        assert!(!coverart.images[0].back);
    } else {
        panic!();
    }
}

#[test]
fn should_get_release_group_coverart_after_fetch() {
    let echoes = ReleaseGroup::fetch()
        .id("ccdb3c9b-67e8-46f5-803f-026ef815ceea")
        .execute()
        .expect("Unable to get release");

    let echoes_coverart = echoes
        .get_coverart()
        .execute()
        .expect("Unable to get coverart");

    if let CoverartResponse::Json(coverart) = echoes_coverart {
        assert!(coverart.images[0].front);
        assert!(!coverart.images[0].back);
    } else {
        panic!();
    }
}