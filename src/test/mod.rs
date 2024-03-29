pub mod controllers;
pub mod utils;
pub mod services;

use diesel::PgConnection;
use diesel::delete;
use diesel::RunQueryDsl;

use crate::schema::events::dsl::*;
use crate::schema::quotes::dsl::*;
use crate::schema::tickers::dsl::*;
use crate::schema::calendars::dsl::*;
use crate::schema::companies::dsl::*;
use crate::schema::currencies::dsl::*;
use crate::schema::segments::dsl::*;
use crate::schema::subsectors::dsl::*;
use crate::schema::sectors::dsl::*;
use crate::schema::holidays::dsl::*;
use crate::schema::indicators::dsl::*;

pub fn clean_database(conn: &mut PgConnection) {
    delete(quotes)
        .execute(conn)
        .expect("Failed to delete quotes");

    delete(events)
        .execute(conn)
        .expect("Failed to delete events");

    delete(tickers)
        .execute(conn)
        .expect("Failed to delete tickers");

    delete(holidays)
        .execute(conn)
        .expect("Failed to delete holidays");

    delete(calendars)
        .execute(conn)
        .expect("Failed to delete calendars");

    delete(indicators)
        .execute(conn)
        .expect("Failed to delete calendars");

    delete(companies)
        .execute(conn)
        .expect("Failed to delete companies");

    delete(currencies)
        .execute(conn)
        .expect("Failed to delete Currencies");

    delete(segments)
        .execute(conn)
        .expect("Failed to delete Segments");
    
    delete(subsectors)
        .execute(conn)
        .expect("Failed to delete Subsectors");

    delete(sectors)
        .execute(conn)
        .expect("Failed to delete Sectors");
}
