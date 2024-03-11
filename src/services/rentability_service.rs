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
        RentabilityService {
            conn: conn,
            business_calendar: business_calendar
        }
    }

    fn get_rentability(&mut self, ticker_id_value: i32, today_close: BigDecimal, last: NaiveDate) -> BigDecimal {
        println!("Insert new rentability quotes!!!!");
        
        let quotes_list: Vec<Quote> = quotes::dsl::quotes
            .filter(quotes::dsl::ticker_id.eq(ticker_id_value))
            .filter(quotes::dsl::date.eq(last))
            .select(Quote::as_select())
            .limit(1)
            .load(self.conn)
            .expect("Error loading quotes");

        if quotes_list.len() == 0 {
            return BigDecimal::from_str("0.0").unwrap();
        } else {
            let quotes_close = quotes_list[0].close.clone();
            let value = today_close / quotes_close + BigDecimal::from_str("-1.0").unwrap();
            return value;
        }
    }

    fn get_date_rentability(&mut self, id_value: i32, date_value: NaiveDate, close_value: BigDecimal, days: i32) -> BigDecimal {
        let date_rent = self.business_calendar.advance(date_value, days.into() );
        let change_5days_value = self.get_rentability(
            id_value, 
            close_value.clone(),
            NaiveDate::parse_from_str(&date_rent, "%Y-%m-%d").unwrap()
        );
        return change_5days_value;
    }

    fn get_month_rentability(&mut self, id_value: i32, date_value: NaiveDate, close_value: BigDecimal) -> BigDecimal {
        let date_rent = NaiveDate::from_ymd_opt(date_value.year(), date_value.month0(), 01).unwrap();
        let change_value = self.get_rentability(
            id_value, 
            close_value.clone(),
            date_rent
        );
        return change_value;
    }

    fn get_year_rentability(&mut self, id_value: i32, date_value: NaiveDate, close_value: BigDecimal) -> BigDecimal {
        let date_rent = NaiveDate::from_ymd_opt(date_value.year(), 01, 01).unwrap();
        let change_value = self.get_rentability(
            id_value, 
            close_value.clone(),
            date_rent
        );
        return change_value;
    }

    fn get_begin_rentability(&mut self, id_value: i32, date_rent: NaiveDate, close_value: BigDecimal) -> BigDecimal {
        let change_value = self.get_rentability(id_value, close_value.clone(), date_rent);
        return change_value;
    }

    pub fn quote_rentability(&mut self, ticker_id_value: i32, quote_params: QuoteParams) -> NewQuote {
        let change_24hrs_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -1);

        let change_5days_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -5);
        let change_7days_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -7);

        let change_month_value = self.get_month_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone() );

        let change_1month_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -30);
        let change_12month_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -365);

        let change_1year_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -365);
        let change_2year_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -365 * 2);
        let change_3year_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -365 * 3);
        let change_4year_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -365 * 4);
        let change_5year_value = self.get_date_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone(), -365 * 5);

        let change_year_value = self.get_year_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone());
        let change_begin_value = self.get_begin_rentability(ticker_id_value, quote_params.date.clone(), quote_params.close.clone());

        return NewQuote {
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
            change_begin: change_begin_value,
            daily_factor: change_24hrs_value, 
            accumulated_factor: BigDecimal::from_str("0").unwrap()
        };

    }

}
