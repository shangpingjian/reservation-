use async_trait::async_trait;
use sqlx::{query, Row};
use sqlx::postgres::types::PgRange;
use abi::{Reservation, ReservationQuery, Error};
use crate::{ReservetionManager, Rsvp, ReservationId};
use chrono::{DateTime, Utc};


#[async_trait]
impl Rsvp for ReservetionManager{
    async fn reserve(&self, mut rsvp: Reservation) -> Result<Reservation, Error> {
        if rsvp.start.is_none() || rsvp.end.is_none(){
            return Err(Error::InvalidTime);
        }

        let start = abi::convert_to_utc_time(rsvp.start.as_ref().unwrap().clone());
        let end = abi::convert_to_utc_time(rsvp.end.as_ref().unwrap().clone());
        if start <= end{
            return Err(Error::InvalidTime);
        }
        let timespan:PgRange<DateTime<Utc>> = (start..end).into();

        let status = abi::ReservationStatus::from_i32(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);


        // generate a insert sql for the reservation
        // execute the sql
        let id = sqlx::query(
            "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"
        )
            .bind(rsvp.user_id.clone())
            .bind(rsvp.resource_id.clone())
            .bind(timespan)
            .bind(rsvp.note.clone())
            .bind(status.as_str_name())
            .fetch_one(&self.pool)
            .await?.get(0);

        rsvp.id = id;

        Ok(rsvp)
    }
    async fn change_status(&self, id: ReservationId) -> Result<Reservation, Error> {
        todo!()
    }
    async fn update_note(&self, id: ReservationId, note: String) -> Result<Reservation, Error> {
        todo!()
    }
    async fn delete(&self, id: ReservationId) -> Result<(), Error> {
        todo!()
    }
    async fn get(&self, id: ReservationId) -> Result<Reservation, Error> {
        todo!()


    }
    async fn query(&self, query: ReservationQuery) -> Result<Vec<Reservation>, Error> {
        todo!()
    }

}

