mod reservation;
mod reservation_status;

use std::ops::Bound;

use crate::convert_to_utc_time;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use sqlx::postgres::types::PgRange;

pub fn get_timespan(start: Option<&Timestamp>, end: Option<&Timestamp>) -> PgRange<DateTime<Utc>> {
    let start = convert_to_utc_time(start.as_ref().unwrap());
    let end = convert_to_utc_time(end.as_ref().unwrap());

    PgRange {
        start: Bound::Included(start),
        end: Bound::Excluded(end),
    }
}
