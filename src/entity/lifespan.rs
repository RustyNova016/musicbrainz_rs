use crate::entity::date_string::DateString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(default)]
pub struct LifeSpan {
    pub ended: Option<bool>,
    #[serde(default)]
    pub begin: Option<DateString>,
    #[serde(default)]
    pub end: Option<DateString>,
}
