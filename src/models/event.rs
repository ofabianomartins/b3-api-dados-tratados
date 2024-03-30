use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

use chrono::NaiveDate;
use bigdecimal::BigDecimal;

use crate::schema::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: i32,
    pub ticker_id: i32,
    pub date: NaiveDate,
    pub ex_date: NaiveDate,
    pub liquidation_date: NaiveDate,
    pub type_: String,
    pub factor: BigDecimal,
    pub strike: Option<BigDecimal>
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub ticker_id: i32,
    pub date: NaiveDate,
    pub ex_date: NaiveDate,
    pub liquidation_date: NaiveDate,
    pub type_: &'a str,
    pub factor: BigDecimal,
    pub strike: Option<BigDecimal>
}
