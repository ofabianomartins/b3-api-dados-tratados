use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::delete;

use crate::models::Calendar;
use crate::models::NewCalendar;
use crate::schema::calendars::dsl::*;
use crate::establish_connection;

#[test]
fn test_get_calendars() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut establish_connection();

    delete(calendars)
        .execute(connection)
        .expect("Failed to delete calendars");

    let calendar = NewCalendar { name: "Calendar 2" };
    insert_into(calendars)
        .values(&calendar)
        .returning(Calendar::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/calendars")
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);

    let test = response.into_string().unwrap();
    let calendars_list: Vec<Calendar> = json::from_str(&test).expect("Failed to read JSON");
    assert_eq!(calendars_list.len(), 1); // Expecting three calendars in the response
    
    delete(calendars)
        .execute(connection)
        .expect("Failed to delete calendars");
}

#[test]
fn test_post_calendars() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut establish_connection();

    delete(calendars)
        .execute(connection)
        .expect("Failed to delete calendars");

    // Setup: Define the data for the new calendar
    let new_calendar = NewCalendar {
        // Define the fields of the new calendar here
        name: "Test Calendar",
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/calendars")
        .header(ContentType::JSON)
        .body(json::to_string(&new_calendar).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.status(), Status::Created);


    delete(calendars)
        .execute(connection)
        .expect("Failed to delete calendars");
}

