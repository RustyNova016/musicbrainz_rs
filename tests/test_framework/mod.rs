pub mod null_eq;
use core::str::FromStr;
use std::sync::LazyLock;

use api_bindium::ApiRequest;
use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::ureq::http::Uri;
use musicbrainz_rs::client::MusicBrainzClient;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::test_framework::null_eq::NullEq as _;

pub(crate) static CLIENT: LazyLock<MusicBrainzClient> = LazyLock::new(|| {
    MusicBrainzClient::new(
        "musicbrainz_rs_test_suite/1.0.0 ( https://github.com/RustyNova016/musicbrainz_rs )",
    )
});
pub(crate) async fn check_fetch_query<T>(
    mut request: ApiRequest<JsonParser<T>>,
    expected_url: &str,
    extra: impl Fn(T),
) where
    T: Serialize + DeserializeOwned + Clone + Sync,
{
    let test_json = request.send_async(&CLIENT.api_client).await.unwrap();
    let test_json_val = serde_json::to_value(test_json).unwrap();
    assert_equal_return(test_json_val.clone(), expected_url.to_string()).await;
    assert_round_trip::<T>(test_json_val.clone());

    let value: T = serde_json::from_value(test_json_val).unwrap();
    extra(value);
}

/// Check if the returned value from the api fetch is the same as expected
pub(crate) async fn assert_equal_return(test_json: Value, expected_url: String) {
    let mut request: ApiRequest<JsonParser<serde_json::Value>> = ApiRequest::builder()
        .uri(
            Uri::from_str(&format!(
                "https://musicbrainz.org/ws/2/{expected_url}&fmt=json"
            ))
            .unwrap(),
        )
        .verb(api_bindium::HTTPVerb::Get)
        .build();

    let expected_json = request.send_async(&CLIENT.api_client).await.unwrap();

    if !expected_json.null_eq(&test_json) {
        eprintln!();
        eprintln!("[ERROR - API RESPONSE]");
        eprintln!();
        eprintln!(
            "Control returned:\n{}",
            serde_json::to_string(&expected_json).unwrap()
        );
        eprintln!();
        eprintln!(
            "But the query builder gave this response:\n{}",
            serde_json::to_string(&test_json).unwrap()
        );
        eprintln!();
        eprintln!("[END TEST]");
        panic!()
    }
}

/// Check if the returned json can do a roundtrip
pub(crate) fn assert_round_trip<T>(json: Value)
where
    T: Serialize + DeserializeOwned + Clone,
{
    let deserialized: T = serde_json::from_value(json.clone()).unwrap();
    let processed = serde_json::to_value(deserialized).unwrap();

    if !json.null_eq(&processed) {
        eprintln!();
        eprintln!("[ERROR - ROUNDTRIP]");
        eprintln!();
        eprintln!(
            "The original JSON was:\n{}",
            serde_json::to_string(&json).unwrap()
        );
        eprintln!();
        eprintln!(
            "But we deserialized:\n{}",
            serde_json::to_string(&processed).unwrap()
        );
        eprintln!();
        eprintln!("[END TEST]");
        panic!()
    }
}
