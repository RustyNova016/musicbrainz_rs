use core::marker::PhantomData;

use api_bindium::JsonParser;
use api_bindium::Parser;
use api_bindium::api_response::ureq_response::UreqResponseInner;
use serde::de::DeserializeOwned;
use snafu::ResultExt as _;

use crate::ParsingError;
use crate::api::parser::musicbrainz_result::MusicbrainzResult;
use crate::api::parser::parsing_error::ApiResponseSnafu;
use crate::api::parser::parsing_error::MusicBrainzSnafu;

pub mod musicbrainz_result;
pub mod parsing_error;

pub struct MusicBrainzParser<T>(PhantomData<T>);

impl<T> Parser<UreqResponseInner> for MusicBrainzParser<T>
where
    T: DeserializeOwned + Sized,
{
    type Output = T;
    type Error = ParsingError;

    fn parse(&self, response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        let res: MusicbrainzResult<T> = JsonParser::default()
            .parse(response)
            .context(ApiResponseSnafu)?;

        res.into_result().context(MusicBrainzSnafu)
    }
}

impl<T> Default for MusicBrainzParser<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
