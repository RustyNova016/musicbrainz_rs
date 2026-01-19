use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::prelude::*;

#[macro_rules_attribute::apply(smol_macros::main!)]
async fn main() {
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute_async()
        .await;

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
