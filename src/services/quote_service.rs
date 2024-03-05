use chrono::Datelike;
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

fn get_rentability(ticker_id_value: i32, today_close: BigDecimal, last: NaiveDate) -> BigDecimal {
    let conn = &mut db_connection();
    
    let quotes_list: Vec<Quote> = quotes
        .filter(ticker_id.eq(ticker_id_value))
        .filter(date.eq(last))
        .select(Quote::as_select())
        .load(conn)
        .expect("Error loading quotes");

    if quotes_list.len() == 0 {
        return BigDecimal::from_str("0.0").unwrap();
    } else {
        let quotes_close = quotes_list[0].close.clone();
        let value = today_close / quotes_close + BigDecimal::from_str("-1.0").unwrap();
        return value;
    }
}

fn get_date_rentability(id_value: i32, date_value: NaiveDate, close_value: BigDecimal, days: i32) -> BigDecimal {
    let mut business_date = crate::utils::business_calendar::BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date_rent = business_date.advance(date_value, (-1 * days).into() );
    let change_5days_value = get_rentability(
        id_value, 
        close_value.clone(),
        NaiveDate::parse_from_str(&date_rent, "%Y-%m-%d").unwrap()
    );
    return change_5days_value;
}

fn get_month_rentability(id_value: i32, date_value: NaiveDate, close_value: BigDecimal) -> BigDecimal {
    let date_rent = NaiveDate::from_ymd_opt(date_value.year(), date_value.month0(), 01).unwrap();
    let change_value = get_rentability(
        id_value, 
        close_value.clone(),
        date_rent
    );
    return change_value;
}

fn get_year_rentability(id_value: i32, date_value: NaiveDate, close_value: BigDecimal) -> BigDecimal {
    let date_rent = NaiveDate::from_ymd_opt(date_value.year(), 01, 01).unwrap();
    let change_value = get_rentability(
        id_value, 
        close_value.clone(),
        date_rent
    );
    return change_value;
}

pub fn process_quote(message: &str) {
    let conn = &mut db_connection();

    let quote_params: QuoteParams = rocket::serde::json::from_str(&message).unwrap();
    let ticker: Ticker = tickers
        .filter(symbol.eq(quote_params.symbol))
        .select(Ticker::as_select())
        .first(conn)
        .expect("Error loading tickers");
    let ticker_id_value = ticker.id;

    let change_24hrs_value = get_date_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone(),
        1
    );

    let change_5days_value = get_date_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone(),
        5
    );

    let change_10days_value = get_date_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone(),
        10
    );

    let change_1week_value = get_date_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone(),
        7
    );

    let change_1month_value = get_date_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone(),
        30
    );

    let change_1year_value = get_date_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone(),
        365
    );

    let change_2year_value = get_date_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone(),
        365 * 2
    );

    let change_5year_value = get_date_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone(),
        365 * 5
    );

    let change_month_value = get_month_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone()
    );

    let change_year_value = get_year_rentability(
        ticker_id_value,
        quote_params.date.clone(),
        quote_params.close.clone()
    );

    let new_quote = NewQuote {
        ticker_id: ticker_id_value,
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
        change_24hrs: change_24hrs_value.clone(),
        change_5days: change_5days_value,
        change_10days: change_10days_value,
        change_1week: change_1week_value,
        change_1month: change_1month_value,
        change_1year: change_1year_value,
        change_2year: change_2year_value,
        change_5year: change_5year_value, 
        change_month: change_month_value, 
        change_year: change_year_value,
        daily_factor: change_24hrs_value, 
        accumulated_factor: BigDecimal::from_str("0").unwrap()
    };

    insert_into(quotes)
        .values(&new_quote)
        .returning(Quote::as_returning())
        .get_result(conn)
        .expect("Failed to insert quote!");
}
