use glob::glob;
use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! write_test {
    ($output_filepath:expr, $glob_pattern:literal, $template_path:literal) => {
        let mut output_file = File::create($output_filepath).expect("failed to write test file");

        glob($glob_pattern)
            .expect("failed to read glob pattern")
            .map(|entry| entry.unwrap())
            .for_each(|path| {
                let mut components = path
                    .iter()
                    .skip(3)
                    .map(|component| component.to_str().unwrap());
                let request_type = components.next().unwrap();
                let entity_type = components.next().unwrap();
                let test_name = std::iter::once(entity_type)
                    .chain(components)
                    .collect::<Vec<&str>>()
                    .join("_")
                    .replace(".", "_")
                    .replace("-", "_");
                eprintln!("Writing setting test: {:?}", test_name);

                let type_annotation = match entity_type {
                    "annotation" => "musicbrainz_rs::entity::annotation::Annotation",
                    "area" => "musicbrainz_rs::entity::area::Area",
                    "artist" => "musicbrainz_rs::entity::artist::Artist",
                    "cdstub" => "musicbrainz_rs::entity::cdstub::CDStub",
                    "event" => "musicbrainz_rs::entity::event::Event",
                    "genre" => "musicbrainz_rs::entity::genre::Genre",
                    "instrument" => "musicbrainz_rs::entity::instrument::Instrument",
                    "label" => "musicbrainz_rs::entity::label::Label",
                    "place" => "musicbrainz_rs::entity::place::Place",
                    "recording" => "musicbrainz_rs::entity::recording::Recording",
                    "release" => "musicbrainz_rs::entity::release::Release",
                    "release-group" => "musicbrainz_rs::entity::release_group::ReleaseGroup",
                    "series" => "musicbrainz_rs::entity::series::Series",
                    "tag" => "musicbrainz_rs::entity::tag::Tag",
                    "url" => "musicbrainz_rs::entity::url::Url",
                    "work" => "musicbrainz_rs::entity::work::Work",
                    _ => unreachable!(),
                };

                let type_annotation = match request_type {
                    "lookup" => Cow::from(type_annotation),
                    "browse" => Cow::from(format!(
                        "musicbrainz_rs::entity::BrowseResult<{type_annotation}>"
                    )),
                    "search" => Cow::from(format!(
                        "musicbrainz_rs::entity::search::SearchResult<{type_annotation}>"
                    )),
                    _ => unreachable!(),
                };

                writeln!(
                    output_file,
                    include_str!($template_path),
                    type_annotation = type_annotation,
                    test_name = test_name,
                    filepath = path.canonicalize().unwrap().to_str().unwrap(),
                )
                .expect("failed to write test file");
            });
    };
}

fn main() {
    // Make cargo rerun the build script if the data directory changes.
    println!("cargo:rerun-if-changed=tests/serde/data");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    eprintln!("Writing tests to: {out_dir:?}");

    write_test!(
        out_dir.join("lookup.rs"),
        "tests/serde/data/lookup/*/*.json",
        "./tests/serde/roundtrip.rs.in"
    );

    write_test!(
        out_dir.join("browse.rs"),
        "tests/serde/data/browse/*/*.json",
        "./tests/serde/roundtrip.rs.in"
    );

    write_test!(
        out_dir.join("search.rs"),
        "tests/serde/data/search/*/*.json",
        "./tests/serde/roundtrip.rs.in"
    );
}
