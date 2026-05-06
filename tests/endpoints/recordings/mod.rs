use api_bindium::parsers::json::testing::TestingJsonParser;
use musicbrainz_rs::Fetch;
use musicbrainz_rs::entity::recording::Recording;

use crate::CLIENT;

#[test]
pub fn edgecase_1() {
    Recording::fetch()
        .id("b80974e2-3da3-4741-bdf4-f2072184aab8")
        .with_place_relations()
        .as_api_request(&CLIENT)
        .unwrap()
        .set_parser(TestingJsonParser::<Recording>::default())
        .send(&CLIENT.api_client)
        .unwrap()
        .parse()
        .unwrap();
}
