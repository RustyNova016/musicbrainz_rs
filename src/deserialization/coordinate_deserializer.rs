use crate::entity::place::Coordinate;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

struct CoordinateVisitor;

impl Visitor<'_> for CoordinateVisitor {
    type Value = Coordinate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a floating point number or a string")
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Coordinate::from(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Coordinate::from(value))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Coordinate::from(value))
    }
}

impl Serialize for Coordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            Self::StringCoordinate(value) => serializer.serialize_str(value),
            Self::FloatCoordinate(value) => serializer.serialize_f64(*value),
        }
    }
}

impl<'de> Deserialize<'de> for Coordinate {
    fn deserialize<D>(deserializer: D) -> Result<Coordinate, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CoordinateVisitor)
    }
}
