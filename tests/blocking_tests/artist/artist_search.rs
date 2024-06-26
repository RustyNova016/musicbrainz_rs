use musicbrainz_rs_nova::entity::artist::*;
use musicbrainz_rs_nova::Search;

#[test]
fn should_search_artist() {
    let query = ArtistSearchQuery::query_builder()
        .artist("Nirvana")
        .and()
        .artist_type("Group")
        .build();

    let result = Artist::search(query).execute().unwrap();

    assert!(!result.entities.is_empty());
}
