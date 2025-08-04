pub mod null_eq;
use std::sync::LazyLock;

use musicbrainz_rs::client::MusicBrainzClient;
use musicbrainz_rs::ApiRequest;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::test_framework::null_eq::NullEq as _;

pub(crate) static CLIENT: LazyLock<MusicBrainzClient> = LazyLock::new(|| {
    MusicBrainzClient::new(
        "musicbrainz_rs_test_suite/1.0.0 ( https://github.com/RustyNova016/musicbrainz_rs )",
    ).unwrap()
});
pub(crate) async fn check_fetch_query<T>(request: ApiRequest, expected_url: &str, extra: impl Fn(T))
where
    T: Serialize + DeserializeOwned + Clone,
{
    let test_json = request.get_json(&CLIENT).await.unwrap();
    assert_equal_return(test_json.clone(), expected_url.to_string()).await;
    assert_round_trip::<T>(test_json.clone());

    let value: T = serde_json::from_value(test_json).unwrap();
    extra(value);
}

/// Check if the returned value from the api fetch is the same as expected
pub(crate) async fn assert_equal_return(test_json: Value, expected_url: String) {
    let request = ApiRequest::new(format!(
        "https://musicbrainz.org/ws/2/{expected_url}&fmt=json"
    ));
    let expected_json = request.get_json(&CLIENT).await.unwrap();

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
