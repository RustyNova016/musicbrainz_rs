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
    pub fn new(string: String) -> Self {
        Self(string)
    }

    pub fn into_naive_date(
        &self,
        default_year: u32,
        default_month: u8,
        default_day: u8,
    ) -> Result<NaiveDate, chrono::ParseError> {
        let mut elements = self.0.split("-");

        let year = elements
            .next()
            .and_then(|date| {
                if date == "????" || date.is_empty() {
                    None
                } else {
                    Some(date.to_string())
                }
            })
            .unwrap_or_else(|| default_year.to_string());

        let month = elements
            .next()
            .and_then(|date| {
                if date == "??" || date.is_empty() {
                    None
                } else {
                    Some(date.to_string())
                }
            })
            .unwrap_or_else(|| default_month.to_string());

        let day = elements
            .next()
            .and_then(|date| {
                if date == "??" || date.is_empty() {
                    None
                } else {
                    Some(date.to_string())
                }
            })
            .unwrap_or_else(|| default_day.to_string());

        NaiveDate::parse_from_str(&format!("{year}-{month}-{day}"), FORMAT)
    }
}

impl<T: Display> From<T> for DateString {
    fn from(value: T) -> Self {
        DateString(value.to_string())
    }
}

impl From<DateString> for String {
    fn from(value: DateString) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::entity::date_string::DateString;
    use crate::entity::date_string::FORMAT;

    #[test]
    fn should_parse() {
        compare_dates("", "2000-10-18");
        compare_dates("2025", "2025-10-18");
        compare_dates("????-07", "2000-07-18");
        compare_dates("2025-07", "2025-07-18");
        compare_dates("????-??-11", "2000-10-11");
        compare_dates("2025-??-11", "2025-10-11");
        compare_dates("????-07-11", "2000-07-11");
        compare_dates("2025-07-11", "2025-07-11");
    }

    fn compare_dates(date_string: &str, naive_string: &str) {
        assert_eq!(
            DateString::from(date_string)
                .into_naive_date(2000, 10, 18)
                .unwrap(),
            NaiveDate::parse_from_str(naive_string, FORMAT).unwrap()
        )
    }
}
