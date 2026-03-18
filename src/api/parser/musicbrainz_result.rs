use crate::entity::api::MusicbrainzError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum MusicbrainzResult<T> {
    Ok(T),
    Err(MusicbrainzError),
}

impl<T> MusicbrainzResult<T> {
    pub fn into_result(self) -> Result<T, MusicbrainzError> {
        match self {
            MusicbrainzResult::Ok(val) => Ok(val),
            MusicbrainzResult::Err(err) => Err(err),
        }
    }
}
