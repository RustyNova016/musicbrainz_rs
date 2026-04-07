use musicbrainz_rs::client::MusicBrainzClient;

#[macro_rules_attribute::apply(smol_macros::main!)]
async fn main() {
    let client =
        MusicBrainzClient::new("MyAwesomeTagger/1.2.0 ( http://myawesometagger.example.com )");

    let url = client
        .endpoints() // Get the endpoint builder of the client
        .ws_2_url() // Get the `/ws/2/url` endpoint
        .ressources(vec!["https://www.monstercat.com/release/742779554338"]) // Multiple URLs can be used
        .release_rels(true) // And you can set the relations here
        .call() // Create the api request
        .unwrap()
        .send_async(&client.api_client) // Send the request
        .await
        .unwrap()
        .parse() // Parse the response
        .unwrap();

    assert!(url.url_count > 0);
}
