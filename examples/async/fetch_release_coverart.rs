use musicbrainz_rs::entity::release::*;
use musicbrainz_rs::entity::CoverartResponse;
use musicbrainz_rs::prelude::*;
use musicbrainz_rs::FetchCoverart;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // CoverArt Query for a Release.
    let in_utero_coverart = Release::fetch_coverart()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute_async()
        .await
        .expect("Unable to get cover art");

    if let CoverartResponse::Json(coverart) = in_utero_coverart {
        assert!(!coverart.images[0].back);
        assert_eq!(
            coverart.images[0].image,
            "http://coverartarchive.org/release/76df3287-6cda-33eb-8e9a-044b5e15ffdd/829521842.jpg"
        );
    } else {
        panic!();
    }

    let in_utero = Release::fetch()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute_async()
        .await
        .expect("Unable to get release");

    // Calling `get_coverart()` method on an already fetched Release entity.
    let in_utero_coverart = in_utero
        .get_coverart()
        .execute_async()
        .await
        .expect("Unable to get coverart");

    if let CoverartResponse::Json(coverart) = in_utero_coverart {
        assert!(!coverart.images[0].back);
        assert_eq!(
            coverart.images[0].image,
            "http://coverartarchive.org/release/76df3287-6cda-33eb-8e9a-044b5e15ffdd/829521842.jpg"
        );
    } else {
        panic!();
    }

    // CoverArt Query Builder to fetch a specific resource.
    let in_utero_500px_front_coverart = Release::fetch_coverart()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .res_500()
        .back()
        .execute_async()
        .await
        .expect("Unable to get cover art");

    if let CoverartResponse::Url(coverart_url) = in_utero_500px_front_coverart {
        println!("{coverart_url}");
    } else {
        panic!();
    }
}
