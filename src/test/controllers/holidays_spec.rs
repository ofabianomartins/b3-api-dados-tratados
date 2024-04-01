use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;

use chrono::NaiveDate;

use crate::connections::db_connection;
use crate::models::holiday::Holiday;
use crate::models::holiday::NewHoliday;
use crate::schema::holidays::dsl::*;

use crate::models::calendar::Calendar;
use crate::models::calendar::NewCalendar;
use crate::schema::calendars::dsl::*;

use crate::test::clean_database;

#[test]
fn test_get_holidays() {
    // Setup: Insert sample data into the test database
    let connection = &mut db_connection();

    clean_database(connection);

    let calendar = NewCalendar { name: "Calendar 3", code: "calendar3" };
    let result_calendar = insert_into(calendars)
        .values(&calendar)
        .returning(Calendar::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");


    let holiday = NewHoliday { 
        name: "Holiday 3",
        date: NaiveDate::parse_from_str("2024-02-03", "%Y-%m-%d").unwrap(),
        calendar_id: result_calendar.id
    };
    insert_into(holidays)
        .values(&holiday)
        .returning(Holiday::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/holidays")
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);

    let test = response.into_string().unwrap();
    let holidays_list: Vec<Holiday> = json::from_str(&test).expect("Failed to read JSON");
    assert_eq!(holidays_list.len(), 1); // Expecting three calendars in the response
    
    clean_database(connection);
}

#[test]
fn test_show_holidays() {
    // Setup: Insert sample data into the test database
    let connection = &mut db_connection();

    clean_database(connection);

    let calendar = NewCalendar { name: "Calendar 3", code: "calendar3" };
    let result_calendar = insert_into(calendars)
        .values(&calendar)
        .returning(Calendar::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    let holiday = NewHoliday { 
        name: "Holiday 3",
        date: NaiveDate::parse_from_str("2024-02-03", "%Y-%m-%d").unwrap(),
        calendar_id: result_calendar.id
    };
    let result_holiday = insert_into(holidays)
        .values(&holiday)
        .returning(Holiday::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(format!("/api/holidays/{}", result_holiday.uuid ))
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);

    clean_database(connection);
}


#[test]
fn test_post_holidays() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);

    let calendar = NewCalendar { name: "Calendar 3", code: "calendar3" };
    let result_calendar = insert_into(calendars)
        .values(&calendar)
        .returning(Calendar::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    // Setup: Define the data for the new calendar
    let new_holiday = NewHoliday {
        name: "Holiday 3",
        date: NaiveDate::parse_from_str("2024-02-03", "%Y-%m-%d").unwrap(),
        calendar_id: result_calendar.id
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/holidays")
        .header(ContentType::JSON)
        .body(json::to_string(&new_holiday).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Created);
    // assert_eq!(response.status(), Status::Created);

    clean_database(connection);
}

#[test]
fn test_delete_holiday() {
    // Setup: Insert sample data into the test database
    let conn = &mut db_connection();

    clean_database(conn);

    let new_calendar = NewCalendar { name: "Calendar 3", code: "calendar3" };
    let result_calendar = insert_into(calendars)
        .values(&new_calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");


    let new_holiday = NewHoliday { 
        name: "Holiday 3",
        date: NaiveDate::parse_from_str("2024-02-03", "%Y-%m-%d").unwrap(),
        calendar_id: result_calendar.id
    };
    let result_holiday = insert_into(holidays)
        .values(&new_holiday)
        .returning(Holiday::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.delete(format!("/api/holidays/{}", result_holiday.uuid ))
        .header(ContentType::JSON)
        .dispatch();

    let result = holidays
        .find(result_holiday.id)
        .select(Holiday::as_select())
        .load(conn)
        .expect("Error loading calendars");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(result.len(), 0); // Expecting three calendars in the response

    clean_database(conn);
}


