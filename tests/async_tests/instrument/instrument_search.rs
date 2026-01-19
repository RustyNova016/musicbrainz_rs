use musicbrainz_rs::Search;
use musicbrainz_rs::entity::instrument::InstrumentType::*;
use musicbrainz_rs::entity::instrument::*;

#[tokio::test]
#[serial_test::serial]
async fn should_search_instrument() {
    let query = InstrumentSearchQuery::query_builder()
        .instrument("octobass")
        .build();

    let result = Instrument::search(query).execute_async().await.unwrap();

    assert!(
        result
            .entities
            .iter()
            .any(|instrument| instrument.instrument_type == StringInstrument)
    );
}
