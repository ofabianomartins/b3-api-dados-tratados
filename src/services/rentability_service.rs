use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::prelude::PgConnection;

use chrono::NaiveDate;
use chrono::Datelike;

use std::str::FromStr;
use bigdecimal::BigDecimal;

use crate::schema::quotes;
use crate::models::Quote;
use crate::models::NewQuote;
use crate::utils::business_calendar::BusinessCalendar;

use crate::services::quote_service::QuoteParams;

pub struct RentabilityService<'a> {
    pub conn: &'a mut PgConnection,
    pub business_calendar: &'a mut BusinessCalendar
}

impl RentabilityService<'_> {
    pub fn new<'a>(conn: &'a mut PgConnection, business_calendar: &'a mut BusinessCalendar) -> RentabilityService<'a> {
        RentabilityService { conn: conn, business_calendar: business_calendar }
    }

    fn get_previous_quote(&mut self, ticker_id_value: i32, today: NaiveDate) -> Option<Quote> {
        let quotes_list = quotes::dsl::quotes
            .filter(quotes::dsl::ticker_id.eq(ticker_id_value))
            .filter(quotes::dsl::date.le(today)) // eq(today))
            .order(quotes::dsl::date.desc())
            .select(Quote::as_select())
            .limit(1)
            .load(self.conn)
            .expect("Error loading quotes");

        if quotes_list.len() == 0 {
            return None;
        } else {
            return Some(quotes_list[0].clone());
        }
    }

    fn get_daily_factor(&mut self, yesterday_close: BigDecimal, today_close: BigDecimal) -> BigDecimal {
        return BigDecimal::from_str("1.0").unwrap() + (
            today_close / yesterday_close + BigDecimal::from_str("-1.0").unwrap()
        )
    }

    fn get_rentability(&mut self, ticker_id_value: i32, today_acc_factor: BigDecimal, date: NaiveDate) -> BigDecimal {
        let quote = self.get_previous_quote(ticker_id_value, date);

        match quote {
            None => BigDecimal::from_str("1.0").unwrap(),
            Some(x) => today_acc_factor / x.accumulated_factor.clone()
        }
    }

    pub fn quote_rentability(&mut self, ticker_id: i32, quote_params: QuoteParams) -> NewQuote {
        let previous_quote_option = self.get_previous_quote(ticker_id, quote_params.date.clone());

        if let Some(previous_quote) = previous_quote_option {
            let date = quote_params.date;

            let daily_factor = self.get_daily_factor(previous_quote.close, quote_params.close.clone());
            let accumulated_factor: BigDecimal = previous_quote.accumulated_factor * daily_factor.clone();

            let yesterday = self.business_calendar.advance(date, -1);
            let change_24hrs_value = self.get_rentability(ticker_id, accumulated_factor.clone(), yesterday);

            let fivedays = self.business_calendar.advance(date, -5);
            let change_5days_value = self.get_rentability(ticker_id, accumulated_factor.clone(), fivedays);

            let sevendays = self.business_calendar.advance(date, -7);
            let change_7days_value = self.get_rentability(ticker_id, accumulated_factor.clone(), sevendays);

            let month_begin = NaiveDate::from_ymd_opt(date.year(), date.month0(), 01).unwrap();
            let change_month_value = self.get_rentability(ticker_id, accumulated_factor.clone(), month_begin);

            let onemonth = self.business_calendar.advance(date, -30);
            let change_1month_value = self.get_rentability(ticker_id, accumulated_factor.clone(), onemonth);

            let twelvemonth = self.business_calendar.advance(date, -365);
            let change_12month_value = self.get_rentability(ticker_id, accumulated_factor.clone(), twelvemonth);

            let oneyear = self.business_calendar.advance(date, -365);
            let change_1year_value = self.get_rentability(ticker_id, accumulated_factor.clone(), oneyear);

            let twoyear = self.business_calendar.advance(date, -365*2);
            let change_2year_value = self.get_rentability(ticker_id, accumulated_factor.clone(), twoyear);

            let threeyear = self.business_calendar.advance(date, -365*3);
            let change_3year_value = self.get_rentability(ticker_id, accumulated_factor.clone(), threeyear);

            let fouryear = self.business_calendar.advance(date, -365*4);
            let change_4year_value = self.get_rentability(ticker_id, accumulated_factor.clone(), fouryear);

            let fiveyear = self.business_calendar.advance(date, -365*3);
            let change_5year_value = self.get_rentability(ticker_id, accumulated_factor.clone(), fiveyear);

            let year = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();
            let change_year_value = self.get_rentability(ticker_id, accumulated_factor.clone(), year);

            return NewQuote {
                ticker_id: ticker_id,
                date: quote_params.date,
                adjust_close: quote_params.close.clone(),
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
                change_24hrs: change_24hrs_value,
                change_5days: change_5days_value,
                change_7days: change_7days_value,
                change_1month: change_1month_value,
                change_12month: change_12month_value,
                change_1year: change_1year_value,
                change_2year: change_2year_value,
                change_3year: change_3year_value,
                change_4year: change_4year_value,
                change_5year: change_5year_value, 
                change_month: change_month_value, 
                change_year: change_year_value,
                change_begin: accumulated_factor.clone(),
                daily_factor: daily_factor, 
                accumulated_factor: accumulated_factor
            };
        } else {
            return NewQuote {
                ticker_id: ticker_id,
                date: quote_params.date,
                adjust_close: quote_params.close.clone(),
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
                change_24hrs: BigDecimal::from_str("1.0").unwrap(),
                change_5days: BigDecimal::from_str("1.0").unwrap(),
                change_7days: BigDecimal::from_str("1.0").unwrap(),
                change_1month: BigDecimal::from_str("1.0").unwrap(),
                change_12month: BigDecimal::from_str("1.0").unwrap(),
                change_1year: BigDecimal::from_str("1.0").unwrap(),
                change_2year: BigDecimal::from_str("1.0").unwrap(),
                change_3year: BigDecimal::from_str("1.0").unwrap(),
                change_4year: BigDecimal::from_str("1.0").unwrap(),
                change_5year: BigDecimal::from_str("1.0").unwrap(),
                change_month: BigDecimal::from_str("1.0").unwrap(),
                change_year: BigDecimal::from_str("1.0").unwrap(),
                change_begin: BigDecimal::from_str("1.0").unwrap(),
                daily_factor: BigDecimal::from_str("1.0").unwrap(),
                accumulated_factor: BigDecimal::from_str("1.0").unwrap(),
            };
        }
    }

}
