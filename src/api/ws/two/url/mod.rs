use core::fmt::Display;
use std::fmt::Write;

use api_bindium::ApiRequest;
use api_bindium::Parser;
use api_bindium::api_response::ureq_response::UreqResponseInner;
use api_bindium::endpoints::UriBuilderError;

use crate::ParsingError;
use crate::api::endpoints::MusicBrainzAPIEnpoints;
use crate::api::parser::MusicBrainzParser;
use crate::entity::url::MultiUrlResponse;
use crate::entity::url::Url;

#[bon::bon]
impl MusicBrainzAPIEnpoints {
    #[builder]
    pub fn ws_2_url(
        &self,
        ressources: Vec<impl Display>,
        #[builder(default)] artist_rels: bool,
        #[builder(default)] label_rels: bool,
        #[builder(default)] release_group_rels: bool,
        #[builder(default)] release_rels: bool,
        #[builder(default)] recording_rels: bool,
        #[builder(default)] url_rels: bool,
        #[builder(default)] work_rels: bool,
    ) -> Result<ApiRequest<UrlResponseParser>, UriBuilderError> {
        let mut endpoint = self
            .endpoint_builder()
            .set_path("/ws/2/url")
            .add_parameter("fmt", "json");

        for ressource in ressources {
            endpoint = endpoint.add_parameter("resource", ressource)
        }

        // There's gotta be a better way to do this, but for now it works
        let mut incs = String::new();

        if artist_rels {
            write!(incs, "artist-rels+").unwrap();
        }

        if label_rels {
            write!(incs, "label-rels+").unwrap();
        }

        if release_group_rels {
            write!(incs, "release-group-rels+").unwrap();
        }

        if release_rels {
            write!(incs, "release-rels+").unwrap();
        }

        if recording_rels {
            write!(incs, "recording-rels+").unwrap();
        }

        if url_rels {
            write!(incs, "url-rels+").unwrap();
        }

        if work_rels {
            write!(incs, "work-rels+").unwrap();
        }

        if incs.ends_with("+") {
            incs.pop();
        }

        endpoint
            .add_parameter("inc", incs)
            .into_api_request(api_bindium::HTTPVerb::Get, UrlResponseParser)
    }
}

pub struct UrlResponseParser;

impl Parser<UreqResponseInner> for UrlResponseParser {
    type Output = MultiUrlResponse;
    type Error = ParsingError;

    fn parse(&self, response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        let res: Result<MultiOrSingle, ParsingError> = MusicBrainzParser::default().parse(response);
        Ok(res?.into_multi())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum MultiOrSingle {
    Single(Url),
    Multi(MultiUrlResponse),
}

impl MultiOrSingle {
    pub fn into_multi(self) -> MultiUrlResponse {
        match self {
            Self::Multi(val) => val,
            Self::Single(val) => MultiUrlResponse {
                urls: vec![val],
                url_count: 1,
                url_offset: 0,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::MusicBrainzClient;

    #[test]
    fn should_request_multiple_urls() {
        let client = MusicBrainzClient::default();

        let result = client
            .endpoints()
            .ws_2_url()
            .ressources(vec![
                "https://www.beatport.com/track/chronomancy/18257269",
                "https://music.apple.com/gb/song/1468407606",
            ])
            .call()
            .unwrap()
            .send(&client.api_client)
            .unwrap()
            .parse()
            .unwrap();

        assert!(result.urls.first().is_some_and(
            |url| url.resource == "https://www.beatport.com/track/chronomancy/18257269"
        ))
    }
}
