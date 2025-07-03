use core::fmt::Display;

use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

/// A string that represent a date.
///
/// This exists for a few reasons:
///  - Musicbrainz can return incomplete dates (Ex: "1965" or "2023-06")
///  - Allows for easy roundtrip
///  - Custom deserializers are expensive
///  - Allows for custom default values
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct DateString(pub String);

const FORMAT: &str = "%Y-%m-%d";

impl DateString {
    pub fn into_naive_date(
        &self,
        default_month: u32,
        default_day: u8,
    ) -> Result<NaiveDate, chrono::ParseError> {
        NaiveDate::parse_from_str(&self.0, FORMAT)
            .or_else(|_err| {
                NaiveDate::parse_from_str(&format!("{}-{default_month}", &self.0), FORMAT)
            })
            .or_else(|_err| {
                NaiveDate::parse_from_str(
                    &format!("{}-{default_month}-{default_day}", &self.0),
                    FORMAT,
                )
            })
    }
}

impl<T: Display> From<T> for DateString {
    fn from(value: T) -> Self {
        DateString(value.to_string())
    }
}
