mod pb;
mod error;

pub use error::*;
pub use pb::*;
use prost_types::Timestamp;
use chrono::{DateTime, NaiveDateTime, Utc};

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as _).unwrap(),
        Utc,
    )
}
