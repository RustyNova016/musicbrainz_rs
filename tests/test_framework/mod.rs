use std::sync::LazyLock;

use musicbrainz_rs::client::MusicBrainzClient;
use reqwest::header;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

pub(crate) static CLIENT: LazyLock<MusicBrainzClient> =
    LazyLock::new(|| MusicBrainzClient::default());

static REQWEST: LazyLock<reqwest::Client> = LazyLock::new(|| {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(
            "musicbrainz_rs_test_suite/1.0.0 ( https://github.com/RustyNova016/musicbrainz_rs )",
        )
        .unwrap(),
    );

    reqwest::Client::builder()
        // see : https://github.com/hyperium/hyper/issues/2136
        .pool_max_idle_per_host(0)
        .default_headers(headers)
        .build()
        .unwrap()
});

pub(crate) async fn check_fetch_query<T, F>(url: &str, result: T, extra: F)
where
    T: Serialize + DeserializeOwned + Clone,
    F: Fn(T),
{
    let fetched: Value = REQWEST
        .get(format!("https://musicbrainz.org/ws/2/{url}&fmt=json"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let result_value = serde_json::to_value(result.clone()).unwrap();

    if !fetched.null_eq(&result_value) {
        eprintln!();
        eprintln!("[ERROR]");
        eprintln!();
        eprintln!(
            "Musicbrainz returned:\n{}",
            serde_json::to_string(&fetched).unwrap()
        );
        eprintln!();
        eprintln!(
            "But we deserialized:\n{}",
            serde_json::to_string(&result_value).unwrap()
        );
        eprintln!();
        eprintln!("[END TEST]");
        panic!()
    }

    extra(result);
    extra(serde_json::from_value(fetched).unwrap())
}

trait NullEq {
    fn null_eq(&self, other: &Self) -> bool;
}

impl NullEq for Value {
    fn null_eq(&self, other: &Self) -> bool {
        match self {
            Value::Null => other.is_null(),

            Value::Array(val) => other.as_array().is_some_and(|val2| val.null_eq(val2)),
            Value::Object(val) => other.as_object().is_some_and(|val2| val.null_eq(val2)),

            Value::Bool(val) => other.as_bool().is_some_and(|val2| val.eq(&val2)),
            Value::Number(val) => other.as_number().is_some_and(|val2| val.eq(val2)),
            Value::String(val) => other.as_str().is_some_and(|val2| val.eq(val2)),
        }
    }
}

impl NullEq for Vec<Value> {
    fn null_eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if !self
                .get(i)
                .is_some_and(|val1| other.get(i).is_some_and(|val2| val1.null_eq(val2)))
            {
                return false;
            }
        }

        true
    }
}

impl NullEq for serde_json::Map<String, Value> {
    fn null_eq(&self, other: &Self) -> bool {
        for (key, value) in self.iter() {
            let other_value = other.get(key).unwrap_or(&Value::Null);

            if !value.null_eq(other_value) {
                return false;
            }
        }

        true
    }
}
