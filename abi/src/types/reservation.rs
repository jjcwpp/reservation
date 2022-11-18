use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;

use super::get_timespan;
use crate::Reservation;

impl Reservation {
    pub fn get_timespan(&self) -> PgRange<DateTime<Utc>> {
        get_timespan(self.start.as_ref(), self.end.as_ref())
    }
}
