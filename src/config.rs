use once_cell::sync::Lazy;
use reqwest::header;
use reqwest::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

#[cfg(feature = "blocking")]
use reqwest::blocking::{Client, RequestBuilder, Response};

#[cfg(feature = "async")]
use reqwest::{Client, RequestBuilder, Response};

const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub(crate) const BASE_URL: &str = "http://musicbrainz.org/ws/2";
pub(crate) const BASE_COVERART_URL: &str = "http://coverartarchive.org";
pub(crate) const FMT_JSON: &str = "?fmt=json";
pub(crate) const PARAM_INC: &str = "&inc=";
pub(crate) const PARAM_OFFSET: &str = "&offset=";
pub(crate) const PARAM_LIMIT: &str = "&limit=";
const HTTP_RATELIMIT_CODE: u16 = 503;

pub(crate) struct MusicBrainzClient(Arc<Mutex<Client>>);

struct MusicBrainzRetries(Arc<Mutex<u32>>);

pub(crate) static HTTP_CLIENT: Lazy<MusicBrainzClient> = Lazy::new(init_http_client);
static HTTP_RETRIES: Lazy<MusicBrainzRetries> = Lazy::new(init_http_retries);

impl MusicBrainzClient {
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let client_ref = Arc::clone(&HTTP_CLIENT.0);
        let client_lock = client_ref.lock().expect("Unable to get musicbrainz client");
        client_lock.get(path)
    }
}

#[cfg(feature = "blocking")]
impl MusicBrainzClient {
    pub(crate) fn send_with_retries(&self, request: RequestBuilder) -> Result<Response, Error> {
        use std::thread;

        let mut retries = *HTTP_RETRIES.0.lock().unwrap();
        loop {
            let request = request.try_clone().unwrap();
            let response = request.send()?;
            if response.status().as_u16() == HTTP_RATELIMIT_CODE && retries > 0 {
                // It seems like the value in the response header is sometimes rounded-off to the
                // lower number, which can be lower than when the server actually accepts the next
                // request. So we add one to the received duration to account for this.
                let headers = response.headers();
                let retry_secs = headers.get("retry-after").unwrap().to_str().unwrap();
                let duration = Duration::from_secs(retry_secs.parse::<u64>().unwrap() + 1);
                thread::sleep(duration);
                retries -= 1;
            } else {
                break Ok(response);
            }
        }
    }
}

#[cfg(feature = "async")]
impl MusicBrainzClient {
    pub(crate) async fn send_with_retries(
        &self,
        request: RequestBuilder,
    ) -> Result<Response, Error> {
        use wasm_timer::Delay;
        let mut retries = *HTTP_RETRIES.0.lock().unwrap();

        #[cfg(feature = "rate_limit")]
        super::rate_limit::wait_for_ratelimit().await;

        loop {
            let request = request.try_clone().unwrap();
            let response = request.send().await?;
            if response.status().as_u16() == HTTP_RATELIMIT_CODE && retries > 0 {
                let headers = response.headers();
                let retry_secs = headers.get("retry-after").unwrap().to_str().unwrap();
                let duration = Duration::from_secs(retry_secs.parse::<u64>().unwrap() + 1);
                let _ = Delay::new(duration).await;
                retries -= 1;
            } else {
                break Ok(response);
            }
        }
    }
}

fn init_http_client() -> MusicBrainzClient {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(DEFAULT_USER_AGENT),
    );

    let client = Client::builder()
        // see : https://github.com/hyperium/hyper/issues/2136
        .pool_max_idle_per_host(0)
        .default_headers(headers)
        .build().expect("Unable to set default user agent, the following values must be set in Cargo.toml : 'name', 'version', 'authors'");

    MusicBrainzClient(Arc::new(Mutex::new(client)))
}

fn init_http_retries() -> MusicBrainzRetries {
    #[cfg(feature = "blocking")]
    let retries = 2;
    #[cfg(feature = "async")]
    let retries = 10;
    MusicBrainzRetries(Arc::new(Mutex::new(retries)))
}

/// Each request sent to MusicBrainz needs to include a User-Agent header,
/// with enough information in the User-Agent to contact the application maintainers.
/// We strongly suggest including your application's version number
/// in the User-Agent string too.
///
/// For more info see [Rate Limiting](https://musicbrainz.org/doc/MusicBrainz_API/Rate_Limiting#Provide_meaningful_User-Agent_strings)
///
/// ## Example
/// ```rust
/// musicbrainz_rs_nova::config::set_user_agent("MyAwesomeTagger/1.2.0 ( http://myawesometagger.example.com )");
/// ```
pub fn set_user_agent(user_agent: &'static str) {
    let client_ref = Arc::clone(&HTTP_CLIENT.0);
    let mut client_lock = client_ref.lock().expect("Unable to set musicbrainz client");

    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(user_agent),
    );

    *client_lock = Client::builder()
        .default_headers(headers)
        .build()
        .expect("Unable to set user agent");
}

pub fn set_default_retries(retries: u32) {
    let retries_ref = Arc::clone(&HTTP_RETRIES.0);
    let mut retries_lock = retries_ref
        .lock()
        .expect("Unable to set musicbrainz client retries");
    *retries_lock = retries;
}
