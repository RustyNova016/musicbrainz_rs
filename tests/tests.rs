use std::sync::LazyLock;

use musicbrainz_rs::MusicBrainzClient;

#[cfg(feature = "async")]
pub(crate) mod test_framework;

#[cfg(feature = "async")]
mod async_tests;

/// New testing tree
mod endpoints;

pub(crate) static CLIENT: LazyLock<MusicBrainzClient> = LazyLock::new(|| {
    MusicBrainzClient::new(
        "musicbrainz_rs_testing/1.0.0 (https://github.com/RustyNova016/musicbrainz_rs)",
    )
});
