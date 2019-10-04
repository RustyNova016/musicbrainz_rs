extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::Fetch;

#[test]
fn set_user_agent_should_work() {
    musicbrainz_rs::config::set_user_agent("my_awesome_app/1.0");

    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute();

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
