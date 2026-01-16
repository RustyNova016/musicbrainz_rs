use musicbrainz_rs::entity::label::*;
use musicbrainz_rs::prelude::*;

fn main() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_tags()
        .with_ratings()
        .execute()
        .unwrap();

    assert!(
        ninja_tune
            .tags
            .unwrap()
            .iter()
            .any(|tag| tag.name == "independent")
    );

    assert!(ninja_tune.rating.is_some());
}
