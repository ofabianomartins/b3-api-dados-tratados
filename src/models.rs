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

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = calendars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Calendar {
    pub id: i32,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = calendars)]
pub struct NewCalendar<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = holidays)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Holiday {
    pub id: i32,
    pub name: String,
    pub date: NaiveDate,
    pub calendar_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = holidays)]
pub struct NewHoliday<'a> {
    pub name: &'a str,
    pub date: NaiveDate,
    pub calendar_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = currencies)]
pub struct NewCurrency<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = tickers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ticker {
    pub id: i32,
    pub symbol: String,
    pub security_type: String,
    pub company_id: i32,
    pub currency_id: i32,
    pub calendar_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = tickers)]
pub struct NewTicker<'a> {
    pub symbol: &'a str,
    pub security_type: &'a str,
    pub company_id: i32,
    pub currency_id: i32,
    pub calendar_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = quotes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Quote {
    pub id: i32,
    pub date: NaiveDate,
    pub ticker_id: i32,
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
	pub change_10days: BigDecimal,
	pub change_1week: BigDecimal,
	pub change_1month: BigDecimal,
	pub change_1year: BigDecimal,
	pub change_2year: BigDecimal,
	pub change_5year: BigDecimal,
	pub change_month: BigDecimal,
	pub change_year: BigDecimal,
	pub daily_factor: BigDecimal,
	pub accumulated_factor: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = quotes)]
pub struct NewQuote {
    pub date: NaiveDate,
    pub ticker_id: i32,
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
	pub change_10days: BigDecimal,
	pub change_1week: BigDecimal,
	pub change_1month: BigDecimal,
	pub change_1year: BigDecimal,
	pub change_2year: BigDecimal,
	pub change_5year: BigDecimal,
	pub change_month: BigDecimal,
	pub change_year: BigDecimal,
	pub daily_factor: BigDecimal,
	pub accumulated_factor: BigDecimal,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = theory_portfolios)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TheoryPortfolio {
    pub id: i32,
    pub name: String,
    pub index_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = theory_portfolios)]
pub struct NewTheoryPortfolio {
    pub name: String,
    pub index_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = theory_portfolio_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TheoryPortfolioTransaction {
    pub id: i32,
    pub date: NaiveDate,
    pub quantity: BigDecimal,
    pub ticker_id: i32,
    pub theory_portfolio_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = theory_portfolio_transactions)]
pub struct NewTheoryPortfolioTransaction {
    pub date: NaiveDate,
    pub quantity: BigDecimal,
    pub ticker_id: i32,
    pub theory_portfolio_id: i32,
}


