use chrono::NaiveDate;

use musicbrainz_rs::entity::area::AreaType::*;
use musicbrainz_rs::entity::area::*;
use musicbrainz_rs::entity::artist::ArtistType::*;
use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::entity::event::{Event, EventType};
use musicbrainz_rs::entity::instrument::InstrumentType::*;
use musicbrainz_rs::entity::instrument::*;
use musicbrainz_rs::entity::label::*;
use musicbrainz_rs::entity::lifespan::*;
use musicbrainz_rs::entity::place::PlaceType::*;
use musicbrainz_rs::entity::place::*;
use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::entity::release::*;
use musicbrainz_rs::entity::release_group::*;
use musicbrainz_rs::entity::series::*;
use musicbrainz_rs::entity::url::*;
use musicbrainz_rs::entity::work::*;
use musicbrainz_rs::prelude::*;

use crate::test_framework::check_fetch_query;
use crate::test_framework::CLIENT;

#[tokio::test]
#[serial_test::serial]
async fn should_get_artist_by_id() {
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute()
        .await;

    assert_eq!(
        nirvana.unwrap(),
        Artist {
            id: String::from("5b11f4ce-a62d-471e-81fc-a69a8278c7da"),
            name: String::from("Nirvana"),
            sort_name: String::from("Nirvana"),
            disambiguation: String::from("1980s\u{2013}1990s US grunge band"),
            artist_type: Some(Group),
            gender: None,
            country: Some("US".to_string()),
            area: Some(Area {
                id: "489ce91b-6658-3307-9877-795b68554c98".to_string(),
                area_type: None,
                type_id: None,
                disambiguation: "".to_string(),
                name: "United States".to_string(),
                sort_name: "United States".to_string(),
                relations: None,
                iso_3166_1_codes: Some(vec!["US".to_string(),]),
                life_span: None,
                tags: None,
                aliases: None,
                genres: None,
                annotation: None,
            }),
            isnis: Some(vec![
                "0000000123486830".to_string(),
                "0000000123487390".to_string()
            ]),
            ipis: Some(vec![]),
            begin_area: None,
            life_span: Some(LifeSpan {
                ended: Some(true),
                begin: Some(NaiveDate::from_ymd_opt(1987, 1, 1).unwrap()),
                end: Some(NaiveDate::from_ymd_opt(1994, 4, 5).unwrap()),
            }),
            tags: None,
            relations: None,
            releases: None,
            recordings: None,
            release_groups: None,
            works: None,
            aliases: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_artist_relations_from_release() {
    let in_utero = Release::fetch()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .with_artist_relations()
        .as_api_request(&CLIENT);

    check_fetch_query(
        in_utero,
        "release/76df3287-6cda-33eb-8e9a-044b5e15ffdd?inc=artist-rels",
        |artist: Artist| {
            let relations = artist
                .relations
                .as_ref()
                .expect("There should have artist relations");

            assert!(!relations.is_empty(), "No relations found")
        },
    )
    .await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_recording_by_id() {
    let polly = Recording::fetch()
        .id("af40d6b8-58e8-4ca5-9db8-d4fca0b899e2")
        .as_api_request(&CLIENT);

    check_fetch_query(polly, "recording/af40d6b8-58e8-4ca5-9db8-d4fca0b899e2?", |_: Recording| {}).await;
}

#[tokio_shared_rt::test(shared)]
async fn should_get_release_group_by_id() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .execute()
        .await;

    assert_eq!(
        in_utero.unwrap(),
        ReleaseGroup {
            id: "2a0981fb-9593-3019-864b-ce934d97a16e".to_string(),
            primary_type_id: Some("f529b476-6e62-324f-b0aa-1f3e33d313fc".to_string()),
            primary_type: Some(ReleaseGroupPrimaryType::Album),
            secondary_type_ids: vec![],
            secondary_types: vec![],
            first_release_date: Some(NaiveDate::from_ymd_opt(1993, 9, 21).unwrap()),
            title: "In Utero".to_string(),
            disambiguation: "".to_string(),
            relations: None,
            artist_credit: None,
            releases: None,
            tags: None,
            aliases: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_release() {
    let in_utero = Release::fetch()
        .id("18d4e9b4-9247-4b44-914a-8ddec3502103")
        .execute()
        .await;

    assert_eq!(
        in_utero.unwrap(),
        Release {
            id: "18d4e9b4-9247-4b44-914a-8ddec3502103".to_string(),
            title: "In Utero".to_string(),
            status_id: Some("4e304316-386d-3409-af2e-78857eec5cfe".to_string()),
            status: Some(ReleaseStatus::Official),
            date: Some(NaiveDate::from_ymd_opt(1993, 1, 1).unwrap()),
            country: Some("US".to_string()),
            quality: Some(ReleaseQuality::Normal),
            barcode: Some("0208314671259".to_string()),
            disambiguation: Some("".to_string()),
            packaging_id: Some("ec27701a-4a22-37f4-bfac-6616e0f9750a".to_string()),
            packaging: Some(ReleasePackaging::JewelCase),
            relations: None,
            artist_credit: None,
            label_info: None,
            media: None,
            release_group: None,
            tags: None,
            aliases: None,
            genres: None,
            annotation: None,
            text_representation: Some(ReleaseTextRepresentation {
                script: Some(ReleaseScript::Latn),
                language: Some(Language::Eng),
            }),
            asin: None,
            cover_art_archive: Some(CoverArtArchiveRelease {
                artwork: true,
                back: true,
                count: 2,
                darkened: false,
                front: true
            })
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_work_by_id() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .execute()
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
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .execute()
        .await;

    assert_eq!(
        ninja_tune.unwrap(),
        Label {
            id: "dc940013-b8a8-4362-a465-291026c04b42".to_string(),
            type_id: Some("7aaa37fe-2def-3476-b359-80245850062d".to_string()),
            label_type: Some(LabelType::OriginalProduction),
            name: "Ninja Tune".to_string(),
            sort_name: Some("Ninja Tune".to_string()),
            disambiguation: Some("London‐based label".to_string()),
            country: Some("GB".to_string()),
            label_code: Some(12885),
            relations: None,
            releases: None,
            aliases: None,
            tags: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_area_by_id() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .execute()
        .await;

    assert_eq!(
        aberdeen.unwrap(),
        Area {
            id: "a640b45c-c173-49b1-8030-973603e895b5".to_string(),
            area_type: Some(City),
            type_id: Some("6fd8f29a-3d0a-32fc-980d-ea697b69da78".to_string()),
            disambiguation: "".to_string(),
            name: "Aberdeen".to_string(),
            sort_name: "Aberdeen".to_string(),
            relations: None,
            iso_3166_1_codes: None,
            tags: None,
            aliases: None,
            genres: None,
            annotation: None,
            life_span: Some(LifeSpan {
                ended: Some(false),
                begin: None,
                end: None,
            }),
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_event_by_id() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .execute()
        .await;

    assert_eq!(
        dour_festival_1989.unwrap(),
        Event {
            id: "73df2f48-383b-4930-bad3-05ba938be578".to_string(),
            name: "Dour 1989".to_string(),
            cancelled: Some(false),
            type_id: Some("b6ded574-b592-3f0e-b56e-5b5f06aa0678".to_string()),
            event_type: Some(EventType::Festival),
            disambiguation: Some("".to_string()),
            time: Some("".to_string()),
            setlist: Some("".to_string()),
            life_span: Some(LifeSpan {
                begin: Some(NaiveDate::from_ymd_opt(1989, 9, 16).unwrap()),
                end: Some(NaiveDate::from_ymd_opt(1989, 9, 16).unwrap()),
                ended: Some(true),
            }),
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
async fn should_get_instrument() {
    let mandoline = Instrument::fetch()
        .id("37fa9bb5-d5d7-4b0f-aa4d-531339ba9c32")
        .execute()
        .await;

    assert_eq!(
        mandoline.unwrap(),
        Instrument {
            id: "37fa9bb5-d5d7-4b0f-aa4d-531339ba9c32".to_string(),
            name: "mandolin".to_string(),
            instrument_type: StringInstrument,
            type_id: "cc00f97f-cf3d-3ae2-9163-041cb1a0d726".to_string(),
            description: Some("".to_string()),
            disambiguation: Some("".to_string()),
            relations: None,
            tags: None,
            aliases: None,
            genres: None,
            annotation: None,
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_place() {
    let blue_note_record = Place::fetch()
        .id("327c29c6-da63-4dc9-a117-1917ee691ce4")
        .execute()
        .await;

    assert_eq!(
        blue_note_record.unwrap(),
        Place {
            id: "327c29c6-da63-4dc9-a117-1917ee691ce4".to_string(),
            name: "Blue Note".to_string(),
            disambiguation: Some("Chicago, 1954-1960".to_string()),
            life_span: Some(LifeSpan {
                begin: Some(NaiveDate::from_ymd_opt(1954, 4, 2).unwrap()),
                end: Some(NaiveDate::from_ymd_opt(1960, 6, 14).unwrap()),
                ended: Some(true),
            }),
            type_id: Some("cd92781a-a73f-30e8-a430-55d7521338db".to_string()),
            place_type: Some(Venue),
            address: Some("3 North Clark Street, Chicago, IL 60602".to_string()),
            area: Some(Area {
                id: "29a709d8-0320-493e-8d0c-f2c386662b7f".to_string(),
                disambiguation: "".to_string(),
                sort_name: "Chicago".to_string(),
                name: "Chicago".to_string(),
                relations: None,
                area_type: None,
                type_id: None,
                iso_3166_1_codes: None,
                life_span: None,
                tags: None,
                aliases: None,
                genres: None,
                annotation: None,
            }),
            coordinates: Some(Coordinates {
                latitude: 41.882_059.into(),
                longitude: (-87.630_881).into()
            }),
            relations: None,
            aliases: None,
            tags: None,
            genres: None,
            annotation: None,
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_series() {
    let la_chanson_du_dimanche = Series::fetch()
        .id("814fb4d5-327f-4e37-8784-f8a707e5f97c")
        .execute()
        .await;

    assert_eq!(
        la_chanson_du_dimanche.unwrap(),
        Series {
            id: "814fb4d5-327f-4e37-8784-f8a707e5f97c".to_string(),
            type_id: "dd968243-7128-30a2-81f0-79843430a8e2".to_string(),
            series_type: SeriesType::RecordingSeries,
            disambiguation: "".to_string(),
            name: "La Chanson du Dimanche — Saison 4".to_string(),
            relations: None,
            tags: None,
            aliases: None,
            genres: None,
            annotation: None,
        }
    );
}

#[tokio_shared_rt::test(shared)]
async fn should_get_url() {
    let svinkels_dot_com = Url::fetch()
        .id("9237f6da-fec6-4b8a-9d52-c7c18e0e2630")
        .execute()
        .await;

    assert_eq!(
        svinkels_dot_com.unwrap(),
        Url {
            resource: "http://www.svinkels.com/".to_string(),
            id: "9237f6da-fec6-4b8a-9d52-c7c18e0e2630".to_string(),
            tags: None,
        }
    );
}
