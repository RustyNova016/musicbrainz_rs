use musicbrainz_rs::entity::area::*;
use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::entity::event::Event;
use musicbrainz_rs::entity::instrument::*;
use musicbrainz_rs::entity::label::*;
use musicbrainz_rs::entity::place::*;
use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::entity::release::*;
use musicbrainz_rs::entity::release_group::*;
use musicbrainz_rs::entity::series::*;
use musicbrainz_rs::entity::url::*;
use musicbrainz_rs::entity::work::*;
use musicbrainz_rs::prelude::*;

use crate::test_framework::CLIENT;
use crate::test_framework::check_fetch_query;

#[tokio::test]
#[serial_test::serial]
async fn should_get_artist_by_id() {
    let data = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .as_api_request(&CLIENT)
        .unwrap();

    check_fetch_query(
        data,
        "artist/5b11f4ce-a62d-471e-81fc-a69a8278c7da?",
        |_: Artist| {},
    )
    .await;
}

//TODO: Put back on
// #[tokio_shared_rt::test(shared)]
// async fn should_get_artist_relations_from_release() {
//     let in_utero = Release::fetch()
//         .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
//         .with_artist_relations()
//         .as_api_request(&CLIENT);

//     check_fetch_query(
//         in_utero,
//         "release/76df3287-6cda-33eb-8e9a-044b5e15ffdd?inc=artist-rels",
//         |artist: Artist| {
//             let relations = artist
//                 .relations
//                 .as_ref()
//                 .expect("There should have artist relations");

//             assert!(!relations.is_empty(), "No relations found")
//         },
//     )
//     .await;
// }

#[tokio_shared_rt::test(shared)]
async fn should_get_recording_by_id() {
    let polly = Recording::fetch()
        .id("af40d6b8-58e8-4ca5-9db8-d4fca0b899e2")
        .as_api_request(&CLIENT);

    check_fetch_query(
        polly.unwrap(),
        "recording/af40d6b8-58e8-4ca5-9db8-d4fca0b899e2?",
        |_: Recording| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_release_group_by_id() {
    let data = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "release-group/2a0981fb-9593-3019-864b-ce934d97a16e?",
        |_: ReleaseGroup| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_release() {
    let data = Release::fetch()
        .id("18d4e9b4-9247-4b44-914a-8ddec3502103")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "release/18d4e9b4-9247-4b44-914a-8ddec3502103?",
        |_: Release| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_work_by_id() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .execute_async()
        .await;

    assert_eq!(
        hotel_california.unwrap(),
        Work {
            id: "22457dc0-ecbf-38f5-9056-11c858530a50".to_string(),
            title: "Hotel California".to_string(),
            type_id: Some("f061270a-2fd6-32f1-a641-f0f8676d14e6".to_string()),
            work_type: Some(WorkType::Song),
            languages: Some(vec!["eng".to_string()]),
            language: Some("eng".to_string()),
            iswcs: Some(vec![
                "T-070.076.790-3".to_string(),
                "T-900.316.249-0".to_string(),
                "T-900.316.251-4".to_string()
            ]),
            attributes: Some(vec![
                WorkAttribute::AcdamId("847166".to_string()),
                WorkAttribute::CompassId("9963301".to_string()),
                WorkAttribute::CompassId("76791".to_string()),
                WorkAttribute::CompassId("6482326".to_string()),
                WorkAttribute::CompassId("4487395".to_string()),
                WorkAttribute::AgaduId("5846801".to_string()),
                WorkAttribute::ApdaycId("2234367".to_string()),
                WorkAttribute::SaycoId("1434791".to_string()),
                WorkAttribute::EcadId("3077".to_string()),
                WorkAttribute::SpaId("21287".to_string()),
                WorkAttribute::PrsTuneCode("20565CR".to_string()),
                WorkAttribute::SabamId("005891100".to_string()),
                WorkAttribute::SacmId("030562613".to_string()),
                WorkAttribute::BumaStemraId("W-000307506".to_string()),
                WorkAttribute::SadaicId("252202".to_string()),
                WorkAttribute::SacemId("76 184 283 11".to_string()),
                WorkAttribute::SgaeId("121.617".to_string()),
                WorkAttribute::CashId("C-1201894773".to_string()),
                WorkAttribute::CashId("C-1001388311".to_string()),
                WorkAttribute::SuisaId("000420 692 06".to_string()),
                WorkAttribute::ApraId("GW00991714".to_string()),
                WorkAttribute::SocanId("10303355".to_string()),
                WorkAttribute::GemaId("910886-001".to_string()),
                WorkAttribute::BmiId("2158644".to_string()),
                WorkAttribute::AscapId("380174570".to_string()),
            ]),
            disambiguation: Some("".to_string()),
            relations: None,
            tags: None,
            aliases: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_label_by_id() {
    let data = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "label/dc940013-b8a8-4362-a465-291026c04b42?",
        |_: Label| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_area_by_id() {
    let data = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "area/a640b45c-c173-49b1-8030-973603e895b5?",
        |_: Area| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_event_by_id() {
    let data = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "event/73df2f48-383b-4930-bad3-05ba938be578?",
        |_: Event| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_instrument() {
    let data = Instrument::fetch()
        .id("37fa9bb5-d5d7-4b0f-aa4d-531339ba9c32")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "instrument/37fa9bb5-d5d7-4b0f-aa4d-531339ba9c32?",
        |_: Instrument| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_place() {
    let data = Place::fetch()
        .id("327c29c6-da63-4dc9-a117-1917ee691ce4")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "place/327c29c6-da63-4dc9-a117-1917ee691ce4?",
        |_: Place| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_series() {
    let data = Series::fetch()
        .id("814fb4d5-327f-4e37-8784-f8a707e5f97c")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "series/814fb4d5-327f-4e37-8784-f8a707e5f97c?",
        |_: Series| {},
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_url() {
    let data = Url::fetch()
        .id("9237f6da-fec6-4b8a-9d52-c7c18e0e2630")
        .as_api_request(&CLIENT);

    check_fetch_query(
        data.unwrap(),
        "url/9237f6da-fec6-4b8a-9d52-c7c18e0e2630?",
        |_: Url| {},
    )
    .await;
}
