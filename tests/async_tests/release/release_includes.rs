use musicbrainz_rs::entity::release::Media;
use musicbrainz_rs::entity::release::Release;
use musicbrainz_rs::prelude::*;

use std::collections::HashSet;

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_release_groups() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .with_release_groups()
        .execute_async()
        .await
        .unwrap();

    assert_eq!(justice_cross.release_group.unwrap().title, "✝");
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_media() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .with_media()
        .execute_async()
        .await
        .unwrap();

    assert!(justice_cross
        .media
        .unwrap()
        .iter()
        .any(|media| media.format.as_ref().unwrap() == "CD"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_discids() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .with_discids()
        .execute_async()
        .await
        .unwrap();

    let medias: Vec<Media> = justice_cross.media.unwrap();
    let cd = medias.first().unwrap();

    assert!(cd
        .discs
        .as_ref()
        .unwrap()
        .iter()
        .any(|disc| disc.id == "TNLYkkUzaFr9BejILb6fsUaDjcg-"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_recordings() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .with_recordings()
        .execute_async()
        .await
        .unwrap();

    let medias: Vec<Media> = justice_cross.media.unwrap();
    let cd = medias.first().unwrap();

    assert!(cd
        .tracks
        .as_ref()
        .unwrap()
        .iter()
        .any(|track| track.title == "D.A.N.C.E."));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_artists() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .with_artists()
        .execute_async()
        .await
        .unwrap();

    assert_eq!(justice_cross.artist_credit.unwrap()[0].name, "Justice");
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_label() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .with_labels()
        .execute_async()
        .await
        .unwrap();

    assert!(justice_cross
        .label_info
        .unwrap()
        .iter()
        .any(|label_info| label_info.label.as_ref().unwrap().name == "Ed Banger Records"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_tags() {
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .with_tags()
        .execute_async()
        .await
        .unwrap();

    assert!(l_ecole_du_micro_d_argent
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "hip hop"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_artist_relations() {
    let in_utero = Release::fetch()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .with_artist_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = in_utero.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "engineer"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_url_relations() {
    let in_utero = Release::fetch()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .with_url_relations()
        .execute_async()
        .await
        .unwrap();

    let relations = in_utero.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "discogs"));
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_level_relations() {
    let become_desert = Release::fetch()
        .id("987f3e2d-22a6-4a4f-b840-c80c26b8b91a")
        .with_recordings()
        .with_work_relations()
        .with_work_level_relations()
        .with_artist_relations()
        .with_recording_level_relations()
        .execute_async()
        .await
        .unwrap();

    let mut target_types = HashSet::new();
    for media in become_desert.media.unwrap().iter() {
        for track in media.tracks.as_ref().unwrap().iter() {
            for relation in track
                .recording
                .as_ref()
                .unwrap()
                .relations
                .as_ref()
                .unwrap()
                .iter()
            {
                target_types.insert(relation.target_type.clone());
            }
        }
    }
    let expected_target_types: HashSet<Option<String>> =
        [Some("artist".to_string()), Some("work".to_string())]
            .iter()
            .cloned()
            .collect();

    assert_eq!(target_types, expected_target_types);
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_aliases() {
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .with_aliases()
        .execute_async()
        .await
        .unwrap();

    assert!(l_ecole_du_micro_d_argent.aliases.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_genres() {
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .with_genres()
        .execute_async()
        .await
        .unwrap();

    assert!(l_ecole_du_micro_d_argent.genres.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn should_get_release_annotation() {
    let pieds_nus_sur_la_braise = Release::fetch()
        .id("bdb24cb5-404b-4f60-bba4-7b730325ae47")
        .with_annotations()
        .execute_async()
        .await
        .unwrap();

    assert!(pieds_nus_sur_la_braise.annotation.is_some());
}
