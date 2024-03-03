use serde::Deserialize;
use serde::Serialize;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;

use chrono::NaiveDate;

use bigdecimal::BigDecimal;
use std::str::FromStr;


use crate::schema::quotes::dsl::*;
use crate::schema::tickers::dsl::*;

use crate::models::Ticker;
use crate::models::Quote;
use crate::models::NewQuote;

use crate::connections::db_connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteParams {
    pub date: NaiveDate,
    pub symbol: String,
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
}

pub fn process_quote(message: &str) {
    let conn = &mut db_connection();

    let business_date = crate::utils::business_calendar::BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    println!("{}", business_date.advance(
        NaiveDate::parse_from_str("2024-03-01", "%Y-%m-%d").unwrap(),
        2
    ));

    let quote_params: QuoteParams = rocket::serde::json::from_str(&message).unwrap();
    let results: Ticker = tickers
        .filter(symbol.eq(quote_params.symbol))
        .select(Ticker::as_select())
        .first(conn)
        .expect("Error loading tickers");

    let new_quote = NewQuote {
        ticker_id: results.id,
        date: quote_params.date,
        close: quote_params.close,
        open: quote_params.open,
        high: quote_params.high,
        low: quote_params.low,
        ask: quote_params.ask,
        bid: quote_params.bid,
        trades: quote_params.trades,
        volume: quote_params.volume, 
        average: quote_params.average,
        adjust: quote_params.adjust,
        change_24hrs: BigDecimal::from_str("0").unwrap(),
        change_5days: BigDecimal::from_str("0").unwrap(),
        change_10days: BigDecimal::from_str("0").unwrap(),
        change_1week: BigDecimal::from_str("0").unwrap(),
        change_1month: BigDecimal::from_str("0").unwrap(),
        change_1year: BigDecimal::from_str("0").unwrap(),
        change_2year: BigDecimal::from_str("0").unwrap(),
        change_5year: BigDecimal::from_str("0").unwrap(),
        change_month: BigDecimal::from_str("0").unwrap(),
        change_year: BigDecimal::from_str("0").unwrap(),
        daily_factor: BigDecimal::from_str("0").unwrap(),
        accumulated_factor: BigDecimal::from_str("0").unwrap()
    };

    insert_into(quotes)
        .values(&new_quote)
        .returning(Quote::as_returning())
        .get_result(conn)
        .expect("Failed to insert quote!");
        
    println!("Quote processed: {}" , results.symbol)
}
