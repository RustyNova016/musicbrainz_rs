/// Utilities for managing development test fixtures
///
/// ## Example
///
/// ```sh
/// cargo run --bin fixtures --features fixtures update
/// ```
///
use std::fs::File;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use glob::glob;
use musicbrainz_rs::client::MUSICBRAINZ_CLIENT;

#[derive(Subcommand)]
enum Command {
    /// update test fixture files
    Update {
        /// fixture paths to update, specified using glob pattern matching.
        #[arg(short, long, default_value = "tests/serde/data/**/*.json")]
        path: String,
    },
}

#[derive(Parser)]
#[command(version, about = "Utilities for managing test fixtures", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

fn glob_paths(x: &str) -> Result<Vec<PathBuf>, glob::PatternError> {
    Ok(glob(x)?
        .filter_map(Result::ok)
        .map(|p| p.to_path_buf())
        .collect())
}

struct Fixture<'q> {
    pub file_path: &'q str,
    pub api_path: &'q str,
    pub params: Vec<(&'q str, &'q str)>,
}

fn fixtures() -> std::vec::Vec<Fixture<'static>> {
    vec![
        Fixture {
            file_path: "tests/serde/data/browse/release/by_label.json",
            api_path: "release",
            params: vec![("label", "47e718e1-7ee4-460c-b1cc-1192a841c6e5")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/area/45f07934-675a-46d6-a577-6f8637a411b1.json",
            api_path: "area/45f07934-675a-46d6-a577-6f8637a411b1",
            params: vec![("inc", "aliases")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/artist/5b11f4ce-a62d-471e-81fc-a69a8278c7da.json",
            api_path: "artist/5b11f4ce-a62d-471e-81fc-a69a8278c7da",
            params: vec![("inc", "aliases")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/artist/db92a151-1ac2-438b-bc43-b82e149ddd50.json",
            api_path: "artist/db92a151-1ac2-438b-bc43-b82e149ddd50",
            params: vec![("inc", "aliases+genres+ratings+tags")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/event/fe39727a-3d21-4066-9345-3970cbd6cca4.json",
            api_path: "event/fe39727a-3d21-4066-9345-3970cbd6cca4",
            params: vec![("inc", "artist-rels+place-rels")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/genre/f66d7266-eb3d-4ef3-b4d8-b7cd992f918b.json",
            api_path: "genre/f66d7266-eb3d-4ef3-b4d8-b7cd992f918b",
            params: vec![],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/instrument/dd430e7f-36ba-49a5-825b-80a525e69190.json",
            api_path: "instrument/dd430e7f-36ba-49a5-825b-80a525e69190",
            params: vec![("inc", "aliases")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/label/46f0f4cd-8aab-4b33-b698-f459faf64190.json",
            api_path: "label/46f0f4cd-8aab-4b33-b698-f459faf64190",
            params: vec![("inc", "aliases")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/place/478558f9-a951-4067-ad91-e83f6ba63e74.json",
            api_path: "place/478558f9-a951-4067-ad91-e83f6ba63e74",
            params: vec![("inc", "aliases")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/recording/b9ad642e-b012-41c7-b72a-42cf4911f9ff.json",
            api_path: "recording/b9ad642e-b012-41c7-b72a-42cf4911f9ff",
            params: vec![("inc", "artist-credits+releases")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/release/59211ea4-ffd2-4ad9-9a4e-941d3148024a.json",
            api_path: "release/59211ea4-ffd2-4ad9-9a4e-941d3148024a",
            params: vec![("inc", "artist-credits+labels+media+discids+recordings")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/release/942b9e7b-3a43-4f22-b200-ab69ee302d16.json",
            api_path: "release/942b9e7b-3a43-4f22-b200-ab69ee302d16",
            params: vec![(
                "inc",
                "aliases+annotation+artist-credits+labels+genres+tags+media+recordings+discids+ratings+isrcs+collections",
            )],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/release/987f3e2d-22a6-4a4f-b840-c80c26b8b91a.json",
            api_path: "release/987f3e2d-22a6-4a4f-b840-c80c26b8b91a",
            params: vec![(
                "inc",
                "artist-credits+labels+media+recordings+artist-rels+recording-rels+recording-level-rels+work-rels+work-level-rels",
            )],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/release/b1dc9838-adf3-43f2-93f9-802b46e5fe59.json",
            api_path: "release/b1dc9838-adf3-43f2-93f9-802b46e5fe59",
            params: vec![(
                "inc",
                "aliases+annotation+artist-credits+genres+tags+collections+labels+media+discids+recordings+release-groups+ratings+isrcs+artist-rels+release-rels+recording-rels+recording-level-rels+work-rels+work-level-rels",
            )],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/release/c9d52105-5c20-3216-bc1b-e54918f8f688.json",
            api_path: "release/c9d52105-5c20-3216-bc1b-e54918f8f688",
            params: vec![(
                "inc",
                "aliases+annotation+artist-credits+genres+tags+collections+labels+media+discids+recordings+release-groups+ratings+isrcs+artist-rels+release-rels+recording-rels+recording-level-rels+work-rels+work-level-rels+label-rels+place-rels+url-rels+release-group-rels+release-group-level-rels",
            )],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/release-group/c9fdb94c-4975-4ed6-a96f-ef6d80bb7738.json",
            api_path: "release-group/c9fdb94c-4975-4ed6-a96f-ef6d80bb7738",
            params: vec![("inc", "artist-credits+releases")],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/series/c5dd4f78-4160-458a-9edc-f87c5ebbc700.json",
            api_path: "series/c5dd4f78-4160-458a-9edc-f87c5ebbc700",
            params: vec![(
                "inc",
                "aliases+annotation+genres+tags+label-rels+release-rels+url-rels",
            )],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/url/46d8f693-52e4-4d03-936f-7ca8459019a7.json",
            api_path: "url/46d8f693-52e4-4d03-936f-7ca8459019a7",
            params: vec![],
        },
        Fixture {
            file_path: "tests/serde/data/lookup/work/b1df2cf3-69a9-3bc0-be44-f71e79b27a22.json",
            api_path: "work/b1df2cf3-69a9-3bc0-be44-f71e79b27a22",
            params: vec![("inc", "artist-rels")],
        },
        Fixture {
            file_path: "tests/serde/data/search/annotation/entity_bdb24cb5-404b-4f60-bba4-7b730325ae47.json",
            api_path: "annotation/bdb24cb5-404b-4f60-bba4-7b730325ae4",
            params: vec![("inc", "artist-rels")],
        },
        Fixture {
            file_path: "tests/serde/data/search/area/Ile-de-France.json",
            api_path: "area",
            params: vec![("query", "Ile de France")],
        },
        Fixture {
            file_path: "tests/serde/data/search/artist/artist_fred_AND_type_group_AND_country_US.json",
            api_path: "artist",
            params: vec![("query", "artist:fred AND type:group AND country:US")],
        },
        Fixture {
            file_path: "tests/serde/data/search/cdstub/title_Doo.json",
            api_path: "cdstub",
            params: vec![("query", "title:Doo")],
        },
        Fixture {
            file_path: "tests/serde/data/search/event/unique.json",
            api_path: "event",
            params: vec![("query", "unique")],
        },
        Fixture {
            file_path: "tests/serde/data/search/instrument/Nose.json",
            api_path: "instrument",
            params: vec![("query", "Nose")],
        },
        Fixture {
            file_path: "tests/serde/data/search/label/Devils_Records.json",
            api_path: "label",
            params: vec![("query", "\"Devil's Records\"")],
        },
        Fixture {
            file_path: "tests/serde/data/search/place/chipping.json",
            api_path: "place",
            params: vec![("query", "chipping")],
        },
        Fixture {
            file_path: "tests/serde/data/search/recording/isrc_GBAHT1600302.json",
            api_path: "recording",
            params: vec![("query", "isrc:GBAHT1600302")],
        },
        Fixture {
            file_path: "tests/serde/data/search/recording/we_will_rock_you_AND_arid_0383dadf-2a4e-4d10-a46a-e9e041da8eb3.json",
            api_path: "recording",
            params: vec![(
                "query",
                "\"we will rock you\" AND arid:0383dadf-2a4e-4d10-a46a-e9e041da8eb3",
            )],
        },
        Fixture {
            file_path: "tests/serde/data/search/release/drivers_license.json",
            api_path: "release",
            params: vec![("query", "\"drivers license\"")],
        },
        Fixture {
            file_path: "tests/serde/data/search/release/release_Schneider_AND_Shake.json",
            api_path: "release",
            params: vec![("query", "release:Schneider AND Shake")],
        },
        Fixture {
            file_path: "tests/serde/data/search/release-group/release_Tenance.json",
            api_path: "release-group",
            params: vec![("query", "release:Tenance")],
        },
        Fixture {
            file_path: "tests/serde/data/search/series/Studio_Brussel.json",
            api_path: "series",
            params: vec![("query", "\"Studio Brussel\"")],
        },
        Fixture {
            file_path: "tests/serde/data/search/tag/shoegaze.json",
            api_path: "tag",
            params: vec![("query", "shoegaze")],
        },
        Fixture {
            file_path: "tests/serde/data/search/url/Hello.json",
            api_path: "url",
            params: vec![("query", "Hello")],
        },
        Fixture {
            file_path: "tests/serde/data/search/work/work_Frozen_AND_arid_4c006444-ccbf-425e-b3e7-03a98bab5997.json",
            api_path: "work",
            params: vec![(
                "query",
                "work:Frozen AND arid:4c006444-ccbf-425e-b3e7-03a98bab5997",
            )],
        },
    ]
}

fn update_fixtures(x: Vec<Fixture>) {
    // use the raw http client to query for raw json result
    let client = &MUSICBRAINZ_CLIENT;
    let apiurl = format!("{}/ws/2", client.musicbrainz_domain);
    let agent = &client.api_client.agent;
    let err_header = "Error encountered while rebuilding test fixtures:\n";

    for query in x {
        let url = format!("https://{apiurl}/{}", query.api_path);

        println!("Updating \"{}\"", query.file_path);
        println!("  querying \"{url}\"");

        let body = agent
            .get(url)
            .query_pairs(query.params)
            .header("User-Agent", "dgkf/musicbrainz_rs (dev fixtures)")
            .header("Accept", "application/json")
            .call()
            .and_then(|resp| resp.into_body().read_to_string());

        let Ok(body) = body else {
            eprintln!("{err_header}{body:?}");
            continue;
        };

        let json: Result<serde_json::Value, _> = serde_json::from_str(&body);
        let Ok(json) = json else {
            eprintln!("{err_header}{json:?}");
            continue;
        };

        let fpath = std::fs::canonicalize(query.file_path).unwrap();
        let Ok(f) = File::create(fpath.clone()) else {
            eprintln!("{err_header}Unable to open file for writing.\n{:?}", &fpath);
            continue;
        };

        serde_json_fmt::JsonFormat::new()
            .colon(": ")
            .unwrap()
            .indent_width(Some(2))
            .ascii(true)
            .format_to_writer(f, &json)
            .unwrap();

        println!("  updated test fixture {:?}", &fpath);

        // very simple rate limiting just for the purpose of getting some fixtures in place
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let fixtures = fixtures();

    match args.command {
        None => (),
        Some(Command::Update { path }) => {
            let paths = glob_paths(&path)?;
            let fixtures = if path.is_empty() {
                fixtures
            } else {
                fixtures
                    .into_iter()
                    .filter(|test| paths.contains(&PathBuf::from(test.file_path)))
                    .collect()
            };
            update_fixtures(fixtures);
        }
    }

    Ok(())
}
