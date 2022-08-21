extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::prelude::*;

#[test]
fn should_get_artist_releases() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_releases()
        .execute()
        .unwrap();

    let releases = john_lee_hooker.releases.unwrap();

    assert!(releases
        .iter()
        .any(|release| release.title == "Sally Mae / Boogie Chillen’"));
}

#[test]
fn should_get_artist_works() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_works()
        .execute()
        .unwrap();

    let works = john_lee_hooker.works.unwrap();

    assert!(works
        .iter()
        .any(|work| work.title == "Baby Please Don't Go"));
}

#[test]
fn should_get_artist_release_groups() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_release_groups()
        .execute()
        .unwrap();

    let release_groups = john_lee_hooker.release_groups.unwrap();

    assert!(release_groups
        .iter()
        .any(|group| group.title == "Travelin’"));
}

#[test]
fn should_get_artist_recordings() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_recordings()
        .execute()
        .unwrap();

    let recordings = john_lee_hooker.recordings.unwrap();

    assert!(recordings
        .iter()
        .any(|recording| recording.title == "(Blues Is) The Healer"));
}

#[test]
fn should_get_artist_aliases() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_aliases()
        .execute()
        .unwrap();

    let aliases = john_lee_hooker.aliases;

    assert!(aliases
        .unwrap()
        .iter()
        .any(|alias| alias.name == "Delta John"));
}

#[test]
fn should_get_artist_artist_relations() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_artist_relations()
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "parent"));
}

#[test]
fn should_get_artist_event_relations() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_event_relations()
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "main performer"));
}

#[test]
fn should_get_artist_url_relations() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_url_relations()
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "BBC Music page"));
}

#[test]
fn should_get_artist_work_relations() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_work_relations()
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "composer"));
}

#[test]
fn should_get_artist_recording_relations() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_recording_relations()
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "arranger"));
}

#[test]
fn should_get_artist_release_relations() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_release_relations()
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "vocal"));
}

#[test]
fn should_get_artist_series_relations() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_series_relations()
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "part of"));
}

#[test]
fn should_get_artist_artist_releases_with_disc_ids() {
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .with_releases_and_discids()
        .execute()
        .unwrap();

    let releases_with_disc_ids = nirvana.releases.unwrap();

    assert!(releases_with_disc_ids
        .iter()
        .any(|release| release.title == "Smells Like Teen Spirit"));
}

#[test]
fn should_get_artist_tags() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_tags()
        .execute()
        .unwrap();

    assert!(john_lee_hooker
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "chicago blues"));
}

#[test]
fn should_get_artist_rating() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_rating()
        .execute()
        .unwrap();

    assert!(john_lee_hooker.rating.is_some());
}

#[test]
fn should_get_artist_genres() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_genres()
        .execute()
        .unwrap();

    assert!(john_lee_hooker
        .genres
        .unwrap()
        .iter()
        .any(|genre| genre.name == "blues"));
}

#[test]
fn should_get_artist_annotation() {
    let franz_joseph_haydn = Artist::fetch()
        .id("c130b0fb-5dce-449d-9f40-1437f889f7fe")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(franz_joseph_haydn.annotation.is_some());
}
