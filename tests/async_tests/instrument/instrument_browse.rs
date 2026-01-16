use musicbrainz_rs::Browse as _;
use musicbrainz_rs::entity::instrument::Instrument;

// TODO: find non empty result
#[tokio::test]
#[serial_test::serial]
async fn browse_instrument_by_collection() {
    let instruments = Instrument::browse()
        .by_collection("6b6dc74a-f779-491a-a0eb-7c1d1ed56fe0")
        .execute_async()
        .await;

    assert!(instruments.is_ok());
    let _instruments = instruments.unwrap();
}
