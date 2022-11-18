use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, Row};
use tokio::sync::mpsc;

use crate::{ReservationManager, Rsvp};

#[allow(unreachable_code)]
#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        let status = abi::ReservationStatus::from_i32(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);
        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timespan();

        let id = sqlx::query("INSERT INTO rsvp.reservations (user_id, reservation_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp::reservation_status RETURNING id")
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?.get(0);

        rsvp.id = id;

        Ok(rsvp)
    }

    async fn change_status(&self, _id: abi::ReservationId) -> Result<abi::Reservation, abi::Error> {
        !todo!()
    }
    async fn update_note(
        &self,
        _id: abi::ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, abi::Error> {
        !todo!()
    }

    async fn delete(&self, _id: abi::ReservationId) -> Result<abi::Reservation, abi::Error> {
        !todo!()
    }
    async fn get(&self, _id: abi::ReservationId) -> Result<abi::Reservation, abi::Error> {
        !todo!()
    }
    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, abi::Error>> {
        !todo!()
    }
}
