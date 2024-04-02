use diesel::Queryable;
use diesel::Selectable;
use diesel::Identifiable;
use diesel::Insertable;

use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = quotes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Quote {
    pub id: i32,
    pub date: NaiveDate,
    pub ticker_id: i32,
    pub adjust_close: BigDecimal,
    pub close: BigDecimal,
	pub open: Option<BigDecimal>,
	pub high: Option<BigDecimal>,
	pub low: Option<BigDecimal>,
	pub average: Option<BigDecimal>,
	pub ask: Option<BigDecimal>,
	pub bid: Option<BigDecimal>,
	pub adjust: Option<BigDecimal>,
	pub volume: Option<BigDecimal>,
	pub trades: Option<BigDecimal>,
	pub change_24hrs: BigDecimal,
	pub change_5days: BigDecimal,
	pub change_7days: BigDecimal,
	pub change_month: BigDecimal,
	pub change_1month: BigDecimal,
	pub change_year: BigDecimal,
	pub change_12month: BigDecimal,
	pub change_1year: BigDecimal,
	pub change_2year: BigDecimal,
	pub change_3year: BigDecimal,
	pub change_4year: BigDecimal,
	pub change_5year: BigDecimal,
	pub change_begin: BigDecimal,
	pub daily_factor: BigDecimal,
	pub accumulated_factor: BigDecimal,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = quotes)]
pub struct NewQuote {
    pub date: NaiveDate,
    pub ticker_id: i32,
    pub adjust_close: BigDecimal,
    pub close: BigDecimal,
	pub open: Option<BigDecimal>,
	pub high: Option<BigDecimal>,
	pub low: Option<BigDecimal>,
	pub average: Option<BigDecimal>,
	pub ask: Option<BigDecimal>,
	pub bid: Option<BigDecimal>,
	pub adjust: Option<BigDecimal>,
	pub volume: Option<BigDecimal>,
	pub trades: Option<BigDecimal>,
	pub change_24hrs: BigDecimal,
	pub change_5days: BigDecimal,
	pub change_7days: BigDecimal,
	pub change_month: BigDecimal,
	pub change_1month: BigDecimal,
	pub change_year: BigDecimal,
	pub change_12month: BigDecimal,
	pub change_1year: BigDecimal,
	pub change_2year: BigDecimal,
	pub change_3year: BigDecimal,
	pub change_4year: BigDecimal,
	pub change_5year: BigDecimal,
	pub change_begin: BigDecimal,
	pub daily_factor: BigDecimal,
	pub accumulated_factor: BigDecimal,
}

