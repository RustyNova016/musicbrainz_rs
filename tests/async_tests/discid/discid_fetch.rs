use musicbrainz_rs::entity::discid::Discid;
use musicbrainz_rs::Fetch;

#[tokio::test]
#[serial_test::serial]
async fn fetch_cd_id() {
    let discids = [
        // Clashing DiscId
        "lwHl8fGzJyLXQR33ug60E8jhf4k-",
        // Example disc id
        "XzPS7vW.HPHsYemQh0HBUGr8vuU-",
        // Part of multiple CD release
        "hUbE3HKkLSkkWDvf4WliXO9OLm4-",
    ];

    for discid in discids {
        assert!(
            Discid::fetch().id(discid).execute().await.is_ok(),
            "{discid} is err",
        );
    }

    assert!(
        Discid::fetch()
            .id("hUbE6HKkLSkkWDvf4WliXO9OLm4-")
            .execute()
            .await
            .is_err(),
        "The diskid does not exist"
    );
}
