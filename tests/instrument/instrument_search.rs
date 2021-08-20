extern crate musicbrainz_rs;

use musicbrainz_rs::entity::instrument::InstrumentType::*;
use musicbrainz_rs::entity::instrument::*;
use musicbrainz_rs::Search;

#[test]
fn should_search_instrument() {
    let query = InstrumentSearchQuery::query_builder()
        .instrument("octobass")
        .build();

    let result = Instrument::search(query).execute().unwrap();

    assert!(result
        .entities
        .iter()
        .any(|instrument| instrument.instrument_type == StringInstrument));
}
