use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use chrono::NaiveDate;
use chrono::NaiveDateTime;

use std::fmt::Debug;

use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = tickers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ticker {
    pub id: i32,
    pub symbol: String,
    pub security_type: String,
    pub unit: String,
    pub creation_date: NaiveDate,
    pub company_id: i32,
    pub currency_id: i32,
    pub calendar_id: i32,
    pub segment_id: i32,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = tickers)]
pub struct NewTicker<'a> {
    pub symbol: &'a str,
    pub security_type: &'a str,
    pub unit: &'a str,
    pub creation_date: NaiveDate,
    pub company_id: i32,
    pub currency_id: i32,
    pub calendar_id: i32,
    pub segment_id: i32,
}

