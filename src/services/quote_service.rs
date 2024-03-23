use serde::Deserialize;
use serde::Serialize;

use rocket::serde::json;

use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::insert_into;
use diesel::prelude::PgConnection;

use chrono::NaiveDate;
use bigdecimal::BigDecimal;

use crate::schema::tickers;
use crate::schema::quotes;
use crate::models::Ticker;
use crate::models::Quote;
use crate::utils::business_calendar::BusinessCalendar;

use crate::services::rentability_service::RentabilityService;

pub struct QuoteService<'a> {
    pub conn: &'a mut PgConnection,
    pub business_calendar: &'a mut BusinessCalendar
}

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

impl QuoteService<'_> {
    pub fn new<'a>(conn: &'a mut PgConnection, business_calendar: &'a mut BusinessCalendar) -> QuoteService<'a> {
        QuoteService { conn: conn, business_calendar: business_calendar }
    }

    pub fn process_quote(&mut self, message: &str) {
        let quote_params: QuoteParams = json::from_str(&message).unwrap();
        let symbol_param = quote_params.symbol.clone();
        let ticker_lists: Vec<Ticker> = tickers::dsl::tickers
            .filter(tickers::dsl::symbol.eq(symbol_param.clone()))
            .select(Ticker::as_select())
            .limit(1)
            .load(self.conn)
            .expect("Error loading tickers");

        if ticker_lists.len() == 0 {
            println!("Symbol {} not found!", symbol_param);
        } else {
            let mut rentability_service = RentabilityService::new(self.conn, self.business_calendar);
            let date = quote_params.date.clone();
            let new_quote = rentability_service.quote_rentability(ticker_lists[0].id, quote_params);

            insert_into(quotes::dsl::quotes)
                .values(&new_quote)
                .returning(Quote::as_returning())
                .get_result(self.conn)
                .expect("Failed to insert quote!");

            println!("Processed message: {} - {}", symbol_param, date);
        }
    }

}
