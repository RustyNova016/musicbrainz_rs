pub(crate) const DEFAULT_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub(crate) const BASE_URL: &str = "musicbrainz.org";
pub(crate) const BASE_COVERART_URL: &str = "https://coverartarchive.org";
pub(crate) const FMT_JSON: &str = "?fmt=json";
pub(crate) const PARAM_INC: &str = "&inc=";
pub(crate) const PARAM_OFFSET: &str = "&offset=";
pub(crate) const PARAM_LIMIT: &str = "&limit=";
pub(crate) const HTTP_RATELIMIT_CODE: u16 = 503;
