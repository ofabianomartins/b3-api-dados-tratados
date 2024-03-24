use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::prelude::PgConnection;

use chrono::NaiveDate;
use chrono::Datelike;

use std::str::FromStr;
use bigdecimal::BigDecimal;

use crate::schema::events;
use crate::schema::quotes;

use crate::models::Event;
use crate::models::Ticker;
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

    fn normalize_close(&mut self, close: BigDecimal, ticker_id: i32, date: NaiveDate) -> BigDecimal {
        let provents_list: Vec<Event> = events::dsl::events
            .filter(events::dsl::ticker_id.eq(ticker_id.clone()))
            .filter(events::dsl::type_.eq_any(vec!["DIVIDEND", "INTEREST_ON_OWN_CAPITAL_ISSUE"]))
            .select(Event::as_select())
            .load(self.conn)
            .expect("Error loading tickers");

        let splits_list: Vec<Event> = events::dsl::events
            .filter(events::dsl::ticker_id.eq(ticker_id.clone()))
            .filter(events::dsl::type_.eq_any(vec!["SPLIT", "INVERSE_SPLIT"]))
            .select(Event::as_select())
            .load(self.conn)
            .expect("Error loading tickers");


        let mut result_close = close;
        for item in &provents_list {
            let mut result_factor = item.factor.clone();
            for item2 in &splits_list {
                if item2.type_.as_str() == "SPLIT" {
                    result_factor = result_factor / item2.factor.clone();
                }
                if item2.type_.as_str().eq("INVERSE_SPLIT") {
                    result_factor = result_factor * item2.factor.clone();
                }
            }
            result_close = result_close + result_factor;
        }

        return result_close;
    }

    fn normalize_yesterday_close(&mut self, close: BigDecimal, ticker_id: i32, date: NaiveDate) -> BigDecimal {
        let splits_list: Vec<Event> = events::dsl::events
            .filter(events::dsl::ticker_id.eq(ticker_id.clone()))
            .filter(events::dsl::type_.eq_any(vec!["SPLIT", "INVERSE_SPLIT"]))
            .select(Event::as_select())
            .load(self.conn)
            .expect("Error loading tickers");


        let mut result_close = close;
        for item in &splits_list {
            if item.type_.as_str() == "SPLIT" {
                result_close = result_close / item.factor.clone();
            }
            if item.type_.as_str().eq("INVERSE_SPLIT") {
                result_close = result_close * item.factor.clone();
            }
        }

        return result_close;

    }

//  def self.normalize_yesterday_close(y_close, security, date)
//    events = CorporateAction.by(security, date).all
//    rsplits = events.select { |e| SPLIT_TYPES.member?(e.type) }
//    subscriptions = events.select { |e| e.type == 'SUBSCRIPTION_RIGHT' }
//    bonuses = events.select { |e| e.type == 'BONUS_ISSUE' }
//
//    # Ajustando o dia anterior caso haja a execução de um direito de subscrição
//    yc_sub = subscriptions.reduce(y_close) do |acc, sr|
//      fin = (sr.strike * sr.factor) + acc
//      (fin / (1.0 + sr.factor)).round(2)
//    end
//
//    # Ajustando o dia anterior casa haja bonificação do emissor
//    yc_sub = bonuses.reduce(yc_sub) do |acc, sr|
//      (acc / (1 + sr.factor)).round(2)
//    end
//
//    # Ajustando o dia anterior caso haja um split ou reverse_split
//    rsplits.reduce(yc_sub) do |acc, s|
//      if s.split?
//        acc / s.factor
//      elsif s.rsplit?
//        acc * s.factor
//      end
//    end
//  end

    fn get_daily_factor(&mut self, yesterday_close: BigDecimal, today_close: BigDecimal, unit: &str) -> BigDecimal {
        if unit == "INDEX_NUMBER" {
            return BigDecimal::from_str("1.0").unwrap() + (
                today_close / yesterday_close + BigDecimal::from_str("-1.0").unwrap()
            )
        } else {
            return BigDecimal::from_str("1.0").unwrap();
        }
    }

    fn get_rentability(&mut self, ticker_id_value: i32, today_acc_factor: BigDecimal, date: NaiveDate) -> BigDecimal {
        let quote = self.get_previous_quote(ticker_id_value, date);

        match quote {
            None => BigDecimal::from_str("1.0").unwrap(),
            Some(x) => today_acc_factor / x.accumulated_factor.clone()
        }
    }

    pub fn quote_rentability(&mut self, ticker: &Ticker, quote_params: QuoteParams) -> NewQuote {
        let previous_quote_option = self.get_previous_quote(ticker.id, quote_params.date.clone());

        if let Some(previous_quote) = previous_quote_option {
            let date = quote_params.date;

            let normalized_close: BigDecimal = self.normalize_close(quote_params.close.clone(), ticker.id, date);
            let normalized_yesterday_close: BigDecimal = self.normalize_yesterday_close(previous_quote.close, ticker.id, date);

            let daily_factor = self.get_daily_factor(normalized_yesterday_close, normalized_close.clone(), ticker.unit.as_str());
            let accumulated_factor: BigDecimal = previous_quote.accumulated_factor * daily_factor.clone();

            let yesterday = self.business_calendar.advance(date, -1);
            let change_24hrs_value = self.get_rentability(ticker.id, accumulated_factor.clone(), yesterday);

            let fivedays = self.business_calendar.advance(date, -5);
            let change_5days_value = self.get_rentability(ticker.id, accumulated_factor.clone(), fivedays);

            let sevendays = self.business_calendar.advance(date, -7);
            let change_7days_value = self.get_rentability(ticker.id, accumulated_factor.clone(), sevendays);

            let month_begin = NaiveDate::from_ymd_opt(date.year(), date.month0(), 01).unwrap();
            let change_month_value = self.get_rentability(ticker.id, accumulated_factor.clone(), month_begin);

            let onemonth = self.business_calendar.advance(date, -30);
            let change_1month_value = self.get_rentability(ticker.id, accumulated_factor.clone(), onemonth);

            let twelvemonth = self.business_calendar.advance(date, -365);
            let change_12month_value = self.get_rentability(ticker.id, accumulated_factor.clone(), twelvemonth);

            let oneyear = self.business_calendar.advance(date, -365);
            let change_1year_value = self.get_rentability(ticker.id, accumulated_factor.clone(), oneyear);

            let twoyear = self.business_calendar.advance(date, -365*2);
            let change_2year_value = self.get_rentability(ticker.id, accumulated_factor.clone(), twoyear);

            let threeyear = self.business_calendar.advance(date, -365*3);
            let change_3year_value = self.get_rentability(ticker.id, accumulated_factor.clone(), threeyear);

            let fouryear = self.business_calendar.advance(date, -365*4);
            let change_4year_value = self.get_rentability(ticker.id, accumulated_factor.clone(), fouryear);

            let fiveyear = self.business_calendar.advance(date, -365*3);
            let change_5year_value = self.get_rentability(ticker.id, accumulated_factor.clone(), fiveyear);

            let year = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();
            let change_year_value = self.get_rentability(ticker.id, accumulated_factor.clone(), year);

            return NewQuote {
                ticker_id: ticker.id,
                date: quote_params.date,
                adjust_close: normalized_close, 
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
                ticker_id: ticker.id,
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
