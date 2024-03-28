use std::str::FromStr;

use crate::rocket;

use chrono::NaiveDate;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;

use crate::models::Sector;
use crate::models::NewSector;
use crate::models::Subsector;
use crate::models::NewSubsector;
use crate::models::Segment;
use crate::models::NewSegment;
use crate::models::Calendar;
use crate::models::NewCalendar;
use crate::models::Company;
use crate::models::NewCompany;
use crate::models::Currency;
use crate::models::NewCurrency;
use crate::models::Ticker;
use crate::models::NewTicker;

use crate::schema::subsectors::dsl::*;
use crate::schema::sectors::dsl::*;
use crate::schema::segments::dsl::*;
use crate::schema::calendars::dsl::*;
use crate::schema::companies::dsl::*;
use crate::schema::currencies::dsl::*;
use crate::schema::tickers::dsl::*;

use crate::connections::db_connection;

use crate::test::clean_database;

fn setup_data(conn: &mut PgConnection) -> Ticker {
    let new_sector = NewSector { name: "Calendar 2" };
    let sector = insert_into(sectors)
        .values(&new_sector)
        .returning(Sector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_subsector = NewSubsector { name: "Calendar 2", sector_id: sector.id };
    let subsector = insert_into(subsectors)
        .values(&new_subsector)
        .returning(Subsector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_segment = NewSegment { name: "Calendar 2", subsector_id: subsector.id };
    let result_segment = insert_into(segments)
        .values(&new_segment)
        .returning(Segment::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_calendar = NewCalendar { name: "Calendar 2", code: "test_calendar2" };
    let result_calendar = insert_into(calendars)
        .values(&new_calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_company = NewCompany { name: "Calendar 2", company_type: "DEFAULT", cnpj: "00.000.000/0001-00" };
    let result_company = insert_into(companies)
        .values(&new_company)
        .returning(Company::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let currency = NewCurrency { name: "Calendar 2", code: "test_calendar2" };
    let result_currency = insert_into(currencies)
        .values(&currency)
        .returning(Currency::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_ticker = NewTicker {
        symbol: "XPTO3",
        security_type: "STOCK",
        unit: "INDEX_NUMBER",
        creation_date: NaiveDate::from_str(&"2024-02-01").expect("Problem to parse"),
        company_id: result_company.id,
        currency_id: result_currency.id,
        calendar_id: result_calendar.id,
        segment_id: result_segment.id
    };
    return insert_into(tickers)
        .values(&new_ticker)
        .returning(Ticker::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");
}

#[test]
fn test_get_tickers() {
    let connection = &mut db_connection();

    clean_database(connection);
    setup_data(connection);

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/tickers")
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let test = response.into_string().unwrap();
    let tickers_list: Vec<Ticker> = json::from_str(&test).expect("Failed to read JSON");
    assert_eq!(tickers_list.len(), 1); // Expecting three calendars in the response
    
    clean_database(connection);
}

#[test]
fn test_show_tickers() {
    let connection = &mut db_connection();

    clean_database(connection);
    let result_ticker = setup_data(connection);

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(format!("/api/tickers/{}", result_ticker.id))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    clean_database(connection);
}

#[test]
fn test_post_tickers() {
    let conn = &mut db_connection();

    clean_database(conn);

    let new_sector = NewSector { name: "Calendar 2" };
    let sector = insert_into(sectors)
        .values(&new_sector)
        .returning(Sector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_subsector = NewSubsector { name: "Calendar 2", sector_id: sector.id };
    let subsector = insert_into(subsectors)
        .values(&new_subsector)
        .returning(Subsector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_segment = NewSegment { name: "Calendar 2", subsector_id: subsector.id };
    let result_segment = insert_into(segments)
        .values(&new_segment)
        .returning(Segment::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_calendar = NewCalendar { name: "Calendar 2", code: "test_calendar2" };
    let result_calendar = insert_into(calendars)
        .values(&new_calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_company = NewCompany { name: "Calendar 2", company_type: "DEFAULT", cnpj: "00.000.000/0001-00" };
    let result_company = insert_into(companies)
        .values(&new_company)
        .returning(Company::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let currency = NewCurrency { name: "Calendar 2", code: "test_calendar2" };
    let result_currency = insert_into(currencies)
        .values(&currency)
        .returning(Currency::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_ticker = NewTicker {
        symbol: "XPTO3",
        security_type: "STOCK",
        unit: "INDEX_NUMBER",
        creation_date: NaiveDate::from_str(&"2024-02-01").expect("Problem to parse"),
        company_id: result_company.id,
        currency_id: result_currency.id,
        calendar_id: result_calendar.id,
        segment_id: result_segment.id
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/tickers")
        .header(ContentType::JSON)
        .body(json::to_string(&new_ticker).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Created);

    clean_database(conn);
}

#[test]
fn test_put_tickers() {
    let conn = &mut db_connection();

    clean_database(conn);

    let new_sector = NewSector { name: "Calendar 2" };
    let sector = insert_into(sectors)
        .values(&new_sector)
        .returning(Sector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_subsector = NewSubsector { name: "Calendar 2", sector_id: sector.id };
    let subsector = insert_into(subsectors)
        .values(&new_subsector)
        .returning(Subsector::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_segment = NewSegment { name: "Calendar 2", subsector_id: subsector.id };
    let result_segment = insert_into(segments)
        .values(&new_segment)
        .returning(Segment::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_calendar = NewCalendar { name: "Calendar 2", code: "test_calendar2" };
    let result_calendar = insert_into(calendars)
        .values(&new_calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_company = NewCompany { name: "Calendar 2", company_type: "DEFAULT", cnpj: "00.000.000/0001-00" };
    let result_company = insert_into(companies)
        .values(&new_company)
        .returning(Company::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let currency = NewCurrency { name: "Calendar 2", code: "test_calendar2" };
    let result_currency = insert_into(currencies)
        .values(&currency)
        .returning(Currency::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_ticker = NewTicker {
        symbol: "XPTO3",
        security_type: "STOCK",
        unit: "INDEX_NUMBER",
        creation_date: NaiveDate::from_str(&"2024-02-01").expect("Problem to parse"),
        company_id: result_company.id,
        currency_id: result_currency.id,
        calendar_id: result_calendar.id,
        segment_id: result_segment.id
    };
    let result_ticker = insert_into(tickers)
        .values(&new_ticker)
        .returning(Ticker::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    let new_ticker2 = NewTicker {
        symbol: "XPTO4",
        security_type: "STOCK",
        unit: "INDEX_NUMBER",
        creation_date: NaiveDate::from_str(&"2024-02-01").expect("Problem to parse"),
        company_id: result_company.id,
        currency_id: result_currency.id,
        calendar_id: result_calendar.id,
        segment_id: result_segment.id
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.put(format!("/api/tickers/{}", result_ticker.id))
        .header(ContentType::JSON)
        .body(json::to_string(&new_ticker2).unwrap())
        .dispatch();

    let result = tickers
        .find(result_ticker.id)
        .select(Ticker::as_select())
        .load(conn)
        .expect("Error loading tickers");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(result.len(), 1); // Expecting three calendars in the response
    assert_eq!(result[0].symbol, "XPTO4"); // Expecting three calendars in the response

    clean_database(conn);
}

#[test]
fn test_delete_subsector() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);

    let result_ticker = setup_data(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.delete(format!("/api/tickers/{}", result_ticker.id ))
        .header(ContentType::JSON)
        .dispatch();

    let result = tickers
        .find(result_ticker.id)
        .select(Ticker::as_select())
        .load(connection)
        .expect("Error loading subsectors");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(result.len(), 0); // Expecting three calendars in the response

    clean_database(connection);
}

