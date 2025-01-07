use musicbrainz_rs_nova::entity::artist::Artist;
use musicbrainz_rs_nova::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    musicbrainz_rs_nova::config::set_user_agent("musicbrainz_rs/0.4");
    musicbrainz_rs_nova::config::set_default_retries(10);

    let artists_on_in_utero_release = Artist::browse()
        .by_release("18d4e9b4-9247-4b44-914a-8ddec3502103")
        .limit(100)
        .execute()
        .await;

    let artists_on_in_utero_release = artists_on_in_utero_release.unwrap();

    artists_on_in_utero_release
        .entities
        .iter()
        .for_each(|artist| println!("{:?}", artist.name));
}
