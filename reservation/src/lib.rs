use async_trait::async_trait;
use sqlx::PgPool;
use tokio::sync::mpsc;

mod manager;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
    async fn change_status(&self, _id: abi::ReservationId) -> Result<abi::Reservation, abi::Error>;
    async fn update_note(
        &self,
        _id: abi::ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, abi::Error>;
    async fn delete(&self, _id: abi::ReservationId) -> Result<abi::Reservation, abi::Error>;
    async fn get(&self, _id: abi::ReservationId) -> Result<abi::Reservation, abi::Error>;
    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, abi::Error>>;
}
