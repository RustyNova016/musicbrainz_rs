use musicbrainz_rs::entity::artist::{Artist, ArtistSearchQuery};
use musicbrainz_rs::prelude::*;

fn main() {
    let query = ArtistSearchQuery::query_builder()
        .artist("Miles Davis")
        .and()
        .country("US")
        .build();

    let query_result = Artist::search(query).execute().unwrap();
    let query_result: Vec<String> = query_result
        .entities
        .iter()
        .map(|artist| artist.name.clone())
        .collect();

    assert!(query_result.contains(&"Miles Davis".to_string()));
    assert!(query_result.contains(&"Miles Davis Quintet".to_string()));
}
