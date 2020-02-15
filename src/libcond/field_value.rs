use std::cmp::Ordering;
use std::str::FromStr;
use std::time;
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum FieldValue {
    String(String),
    Number(u64),
    Duration(time::Duration),
}

impl From<&str> for FieldValue {
    fn from(v: &str) -> FieldValue {
        FieldValue::String(String::from(v))
    }
}

impl From<u64> for FieldValue {
    fn from(u: u64) -> FieldValue {
        FieldValue::Number(u)
    }
}

impl From<time::Duration> for FieldValue {
    fn from(d: Duration) -> FieldValue {
        FieldValue::Duration(d)
    }
}

impl PartialEq<String> for FieldValue {
    fn eq(&self, other: &String) -> bool {
        match self {
            FieldValue::String(s) => s == other,
            FieldValue::Number(d) => {
                if let Ok(v) = u64::from_str(other) {
                    v.eq(d)
                } else {
                    false
                }
            }
            FieldValue::Duration(d) => {
                if let Ok(other_dur) = parse_duration::parse(other) {
                    d == &other_dur
                } else {
                    false
                }
            }
        }
    }
}

impl PartialOrd<String> for FieldValue {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        match self {
            FieldValue::String(s) => Some(s.cmp(other)),
            FieldValue::Number(d) => d.partial_cmp(&u64::from_str(other).ok()?),
            FieldValue::Duration(dur) => {
                let parsed_duration = parse_duration::parse(other);
                if let Ok(other_dur) = parsed_duration {
                    Some(dur.cmp(&other_dur))
                } else {
                    None
                }
            }
        }
    }
}
