use core::fmt;
use core::fmt::Display;
use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

/// A pair of coordinates
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Coordinates {
    pub latitude: serde_json::Value,
    pub longitude: serde_json::Value,
}

/// Place coordinate (e.g., latitude or longitude).
///
/// The MusicBrainz API either returns a string, an integer or a floating point number. This enum abstracts
/// that so that the user does not have to care about this distinction.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Coordinate {
    StringCoordinate(String),
    NumberCoordinate(serde_json::Number),
}

impl Coordinate {
    pub fn as_cow_str(&self) -> Cow<'_, str> {
        match &self {
            Self::StringCoordinate(value) => Cow::from(value.as_str()),
            Self::NumberCoordinate(value) => Cow::from(value.to_string()),
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match &self {
            Self::StringCoordinate(value) => value.as_str().parse::<f64>().ok(),
            Self::NumberCoordinate(value) => value.as_f64(),
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match &self {
            Self::StringCoordinate(value) => value.as_str().parse::<i64>().ok(),
            Self::NumberCoordinate(value) => value.as_i64(),
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::StringCoordinate(value) => value.fmt(f),
            Self::NumberCoordinate(value) => value.fmt(f),
        }
    }
}

impl From<String> for Coordinate {
    fn from(value: String) -> Self {
        Self::StringCoordinate(value)
    }
}

impl From<&str> for Coordinate {
    fn from(value: &str) -> Self {
        Self::StringCoordinate(value.to_string())
    }
}

impl TryFrom<f64> for Coordinate {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self::NumberCoordinate(
            serde_json::Number::from_f64(value).ok_or(())?,
        ))
    }
}

impl From<i64> for Coordinate {
    fn from(value: i64) -> Self {
        Self::NumberCoordinate(serde_json::Number::from(value))
    }
}

#[cfg(test)]
mod test {

    use crate::entity::place::Coordinate;

    #[test]
    fn should_deserialize_coordinate() {
        assert_eq!(
            serde_json::from_str::<Coordinate>(r#""75""#).unwrap(),
            Coordinate::StringCoordinate("75".to_string())
        );
        assert_eq!(
            serde_json::from_str::<Coordinate>(r#"75.12456"#).unwrap(),
            Coordinate::NumberCoordinate(serde_json::Number::from_f64(75.12456).unwrap())
        );
        assert_eq!(
            serde_json::from_str::<Coordinate>(r#"75"#).unwrap(),
            Coordinate::NumberCoordinate(serde_json::Number::from(75))
        );
    }

    #[test]
    fn should_serialize_coordinate() {
        assert_eq!(
            serde_json::to_string(&Coordinate::StringCoordinate("75".to_string())).unwrap(),
            r#""75""#
        );
        assert_eq!(
            serde_json::to_string(&Coordinate::NumberCoordinate(
                serde_json::Number::from_f64(75.12456).unwrap()
            ))
            .unwrap(),
            r#"75.12456"#
        );
        assert_eq!(
            serde_json::to_string(&Coordinate::NumberCoordinate(serde_json::Number::from(75)))
                .unwrap(),
            r#"75"#
        );
    }
}
