//! Test with special edge cases that broke in the past

use api_bindium::ApiClient;
use api_bindium::ApiRequest;
use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::ureq::http::Uri;
use musicbrainz_rs::entity::recording::Recording;

#[test]
fn should_read_integer_coordinates() {
    let mut rec: ApiRequest<JsonParser<Recording>> = ApiRequest::builder()
    .uri(Uri::from_static("https://musicbrainz.org/ws/2/recording/61bab1e9-dab1-4162-a205-98aee62b9ebe?fmt=json&inc=genre-rels+instrument-rels+label-rels+place-rels+recording-rels+recording-rels+release-rels"))
    .verb(api_bindium::HTTPVerb::Get).build();

    rec.send(&ApiClient::builder().build()).unwrap();
}
