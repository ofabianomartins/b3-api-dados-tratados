use diesel::insert_into;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::prelude::PgConnection;

use crate::models::Event;
use crate::models::NewEvent;
use crate::models::Quote;
use crate::models::NewQuote;
use crate::models::Ticker;
use crate::models::NewTicker;
use crate::models::Calendar;
use crate::models::NewCalendar;
use crate::models::Company;
use crate::models::NewCompany;
use crate::models::Currency;
use crate::models::NewCurrency;
use crate::models::Segment;
use crate::models::NewSegment;
use crate::models::Subsector;
use crate::models::NewSubsector;
use crate::models::Sector;
use crate::models::NewSector;

use crate::schema::events;
use crate::schema::quotes;
use crate::schema::tickers;
use crate::schema::calendars::dsl::*;
use crate::schema::companies::dsl::*;
use crate::schema::currencies::dsl::*;
use crate::schema::segments::dsl::*;
use crate::schema::subsectors::dsl::*;
use crate::schema::sectors::dsl::*;

use crate::connections::db_connection;

use crate::utils::business_calendar::BusinessCalendar;
use crate::services::rentability_service::RentabilityService;
use crate::services::quote_service::QuoteParams;

use crate::test::clean_database;

use std::str::FromStr;
use chrono::NaiveDate;
use chrono::Datelike;
use bigdecimal::BigDecimal;

fn setup_data(conn: &mut PgConnection) -> Ticker {
    let new_calendar = NewCalendar { name: "Calendar 2", code: "test_calendar2" };
    let calendar = insert_into(calendars)
        .values(&new_calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_currency = NewCurrency { name: "Real Brasileiro", code: "BRL" };
    let currency = insert_into(currencies)
        .values(&new_currency)
        .returning(Currency::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_company = NewCompany {
        name: "Real Brasileiro", 
        company_type: "DEFAULT",
        cnpj: "00.000.000/0001-00"
    };
    let company = insert_into(companies)
        .values(&new_company)
        .returning(Company::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_sector = NewSector { name: "Sector1" };
    let sector = insert_into(sectors)
        .values(&new_sector)
        .returning(Sector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_subsector = NewSubsector { name: "Subsector1", sector_id: sector.id };
    let subsector = insert_into(subsectors)
        .values(&new_subsector)
        .returning(Subsector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_segment = NewSegment { name: "Segment1", subsector_id: subsector.id };
    let segment = insert_into(segments)
        .values(&new_segment)
        .returning(Segment::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_ticker = NewTicker { 
        symbol: "PETR4", 
        security_type: "STOCK",
        unit: "INDEX_NUMBER",
        creation_date: NaiveDate::from_str(
            &String::from_str("2024-03-01").expect("Date format problem!"),
        ).expect("NaiveDate not fix"),
        company_id: company.id,
        currency_id: currency.id,
        calendar_id: calendar.id,
        segment_id: segment.id 
    };
    let ticker = insert_into(tickers::dsl::tickers)
        .values(&new_ticker)
        .returning(Ticker::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");
    return ticker
}

fn create_quote(
    conn: &mut PgConnection, 
    ticker_id: i32, 
    date: NaiveDate, 
    close: BigDecimal,
    daily_factor: BigDecimal,
    accumulated_factor: BigDecimal
) {
    let new_quote = NewQuote {
        ticker_id: ticker_id,
        date: date,
        adjust_close: close.clone(),
        close: close,
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
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
        daily_factor: daily_factor, 
        accumulated_factor: accumulated_factor
    };

    insert_into(quotes::dsl::quotes)
        .values(&new_quote)
        .returning(Quote::as_returning())
        .get_result(conn)
        .expect("Failed to insert quote!");
}

#[test]
fn test_create_quote() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    // Action: Make a request to the route
    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );
    let mut service = RentabilityService::new(connection, business_calendar);

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: NaiveDate::from_str(
            &String::from_str("2024-03-01").expect("Date format problem!"),
        ).expect("NaiveDate not fix"),
        close: BigDecimal::from_str("1.0").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.open, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.low, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.high, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.average, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.ask, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.bid, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.adjust, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.volume, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.trades, Some(BigDecimal::from_str("1.0").unwrap()));
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_12month, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_year, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_1year, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_2year, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_3year, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_4year, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.change_5year, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.0").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("1.0").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_yesterday() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let yesterday = business_calendar.advance(date, -1);

    create_quote(connection,
        ticker.id,
        yesterday,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("15.0").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("15.0").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("15.0").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("1.5").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_5days_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let fivedays = business_calendar.advance(date, -5);
    let yesterday = business_calendar.advance(date, -1);

    create_quote(connection,
        ticker.id,
        fivedays,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    create_quote(connection,
        ticker.id,
        yesterday,
        BigDecimal::from_str("15.0").unwrap(),
        BigDecimal::from_str("1.5").unwrap(),
        BigDecimal::from_str("1.5").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date,
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.25").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_7days_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let sevendays = business_calendar.advance(date, -7);

    create_quote(connection,
        ticker.id,
        sevendays,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("28.0").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("28.0").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("28.0").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.8").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_month_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let sevendays = NaiveDate::from_ymd_opt(date.year(), date.month0(), 01).unwrap();

    create_quote(connection,
        ticker.id,
        sevendays,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("28.0").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );
    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("28.0").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("28.0").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.8").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_year_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let sevendays = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();

    create_quote(connection,
        ticker.id,
        sevendays,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("28.0").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );
    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("28.0").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("28.0").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.change_year, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("2.8").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.8").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_12months_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let begin_year = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();
    let twelve_months = business_calendar.advance(date, -365);

    create_quote(connection,
        ticker.id,
        twelve_months,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    create_quote(connection,
        ticker.id,
        begin_year,
        BigDecimal::from_str("15.0").unwrap(),
        BigDecimal::from_str("1.5").unwrap(),
        BigDecimal::from_str("1.5").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_year, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_12month, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.25").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_1year_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let begin_year = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();
    let twelve_months = business_calendar.advance(date, -365);

    create_quote(connection,
        ticker.id,
        twelve_months,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    create_quote(connection,
        ticker.id,
        begin_year,
        BigDecimal::from_str("15.0").unwrap(),
        BigDecimal::from_str("1.5").unwrap(),
        BigDecimal::from_str("1.5").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_year, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_12month, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_1year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.25").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_2year_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let begin_year = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();
    let twelve_months = business_calendar.advance(date, -365*2);

    create_quote(connection,
        ticker.id,
        twelve_months,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    create_quote(connection,
        ticker.id,
        begin_year,
        BigDecimal::from_str("15.0").unwrap(),
        BigDecimal::from_str("1.5").unwrap(),
        BigDecimal::from_str("1.5").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_year, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_12month, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_1year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_2year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.25").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_3year_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let begin_year = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();
    let twelve_months = business_calendar.advance(date, -365*3);

    create_quote(connection,
        ticker.id,
        twelve_months,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    create_quote(connection,
        ticker.id,
        begin_year,
        BigDecimal::from_str("15.0").unwrap(),
        BigDecimal::from_str("1.5").unwrap(),
        BigDecimal::from_str("1.5").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_year, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_12month, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_1year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_2year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_3year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.25").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_4year_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let begin_year = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();
    let twelve_months = business_calendar.advance(date, -365*4);

    create_quote(connection,
        ticker.id,
        twelve_months,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    create_quote(connection,
        ticker.id,
        begin_year,
        BigDecimal::from_str("15.0").unwrap(),
        BigDecimal::from_str("1.5").unwrap(),
        BigDecimal::from_str("1.5").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_year, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_12month, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_1year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_2year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_3year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_4year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.25").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_5year_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let begin_year = NaiveDate::from_ymd_opt(date.year(), 01, 01).unwrap();
    let twelve_months = business_calendar.advance(date, -365*5);

    create_quote(connection,
        ticker.id,
        twelve_months,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    create_quote(connection,
        ticker.id,
        begin_year,
        BigDecimal::from_str("15.0").unwrap(),
        BigDecimal::from_str("1.5").unwrap(),
        BigDecimal::from_str("1.5").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.change_24hrs, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_5days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_7days, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_month, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_year, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.change_12month, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_1year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_2year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_3year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_4year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.change_5year, BigDecimal::from_str("2.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.5").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.25").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_dividends_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let yesterday = business_calendar.advance(date, -1);
    let dplus2 = business_calendar.advance(date, 2);

    let new_event = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "DIVIDEND",
        factor: BigDecimal::from_str("-2.5").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    create_quote(connection,
        ticker.id,
        yesterday,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("20.0").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("2.0").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.0").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_interests_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let yesterday = business_calendar.advance(date, -1);
    let dplus2 = business_calendar.advance(date, 2);

    let new_event = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "INTEREST_ON_OWN_CAPITAL_ISSUE",
        factor: BigDecimal::from_str("-2.5").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    create_quote(connection,
        ticker.id,
        yesterday,
        BigDecimal::from_str("10.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("20.0").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("2.0").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.0").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_splits_normalize_close_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let yesterday = business_calendar.advance(date, -1);
    let dplus2 = business_calendar.advance(date, 2);

    let new_event1 = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "DIVIDEND",
        factor: BigDecimal::from_str("-2.5").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event1)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    let new_event2 = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "SPLIT",
        factor: BigDecimal::from_str("2.0").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event2)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    create_quote(connection,
        ticker.id,
        yesterday,
        BigDecimal::from_str("20.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("21.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("2.125").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.125").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_inverse_splits_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let yesterday = business_calendar.advance(date, -1);
    let dplus2 = business_calendar.advance(date, 2);

    let new_event1 = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "DIVIDEND",
        factor: BigDecimal::from_str("-2.5").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event1)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    let new_event2 = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "INVERSE_SPLIT",
        factor: BigDecimal::from_str("2.0").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event2)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    create_quote(connection,
        ticker.id,
        yesterday,
        BigDecimal::from_str("5.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("17.5").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.75").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("1.75").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_splits_normalize_yesterday_close_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let yesterday = business_calendar.advance(date, -1);
    let dplus2 = business_calendar.advance(date, 2);

    let new_event1 = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "DIVIDEND",
        factor: BigDecimal::from_str("-2.5").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event1)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    let new_event2 = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "SPLIT",
        factor: BigDecimal::from_str("2.0").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event2)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    create_quote(connection,
        ticker.id,
        yesterday,
        BigDecimal::from_str("20.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("21.25").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("2.125").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("2.125").unwrap());
    
    clean_database(connection);
}

#[test]
fn test_create_quote_with_inverse_splits_normalize_yesterday_close_past() {
    let connection = &mut db_connection();
    clean_database(connection);
    let ticker = setup_data(connection);

    let business_calendar = &mut BusinessCalendar::new(
        String::from_str("2024-03-01").expect("Date format problem!"),
        String::from_str("2024-03-10").expect("Date format problem!"),
        Vec::new()
    );

    let date = NaiveDate::from_str(
        &String::from_str("2024-03-01").expect("Date format problem!"),
    ).expect("NaiveDate not fix");
    let yesterday = business_calendar.advance(date, -1);
    let dplus2 = business_calendar.advance(date, 2);

    let new_event1 = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "DIVIDEND",
        factor: BigDecimal::from_str("-2.5").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event1)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    let new_event2 = NewEvent {
        ticker_id: ticker.id,
        date: date,
        ex_date: date,
        liquidation_date: dplus2,
        type_: "INVERSE_SPLIT",
        factor: BigDecimal::from_str("2.0").unwrap(),
        strike: None
    };

    insert_into(events::dsl::events)
        .values(&new_event2)
        .returning(Event::as_returning())
        .get_result(connection)
        .expect("Failed to insert quote!");

    create_quote(connection,
        ticker.id,
        yesterday,
        BigDecimal::from_str("5.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap(),
        BigDecimal::from_str("1.0").unwrap()
    );

    let quote_params = QuoteParams {
        symbol: String::from_str("PETR4").unwrap(),
        date: date, 
        close: BigDecimal::from_str("22.5").unwrap(),
        open: Some(BigDecimal::from_str("1.0").unwrap()),
        high: Some(BigDecimal::from_str("1.0").unwrap()),
        low: Some(BigDecimal::from_str("1.0").unwrap()),
        average: Some(BigDecimal::from_str("1.0").unwrap()),
        ask: Some(BigDecimal::from_str("1.0").unwrap()),
        bid: Some(BigDecimal::from_str("1.0").unwrap()),
        adjust: Some(BigDecimal::from_str("1.0").unwrap()),
        volume: Some(BigDecimal::from_str("1.0").unwrap()),
        trades: Some(BigDecimal::from_str("1.0").unwrap()),
    };

    let mut service = RentabilityService::new(connection, business_calendar);
    let new_quote: NewQuote = service.quote_rentability(&ticker, quote_params);

    assert_eq!(new_quote.close, BigDecimal::from_str("22.5").unwrap());
    assert_eq!(new_quote.adjust_close, BigDecimal::from_str("17.5").unwrap());
    assert_eq!(new_quote.daily_factor, BigDecimal::from_str("1.75").unwrap());
    assert_eq!(new_quote.accumulated_factor, BigDecimal::from_str("1.75").unwrap());
    
    clean_database(connection);
}
