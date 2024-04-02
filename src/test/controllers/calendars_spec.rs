use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;

use crate::models::calendar::Calendar;
use crate::models::calendar::NewCalendar;
use crate::schema::calendars;
use crate::connections::db_connection;

use crate::test::clean_database;

fn setup_data(conn: &mut PgConnection) -> Calendar {
    let calendar = NewCalendar { name: "Calendar 2", code: "test_calendar2" };
    return insert_into(calendars::dsl::calendars)
        .values(&calendar)
        .returning(Calendar::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");
}

#[test]
fn test_get_calendars() {
    let connection = &mut db_connection();
    clean_database(connection);
    setup_data(connection);

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
    
    clean_database(connection);
}

#[test]
fn test_show_calendars() {
    let connection = &mut db_connection();
    clean_database(connection);
    let result_calendar = setup_data(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(format!("/api/calendars/{}", result_calendar.id ))
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.len(), 2); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_show_calendar_not_exists() {
    let connection = &mut db_connection();
    clean_database(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(format!("/api/calendars/{}",1000))
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NotFound);
    // assert_eq!(response.len(), 2); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_post_calendars() {
    let connection = &mut db_connection();

    clean_database(connection);

    // Setup: Define the data for the new calendar
    let new_calendar = NewCalendar {
        // Define the fields of the new calendar here
        name: "Test Calendar",
        code: "test_calendar",
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/calendars")
        .header(ContentType::JSON)
        .body(json::to_string(&new_calendar).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Created);
    // assert_eq!(response.status(), Status::Created);

    clean_database(connection);
}

#[test]
fn test_update_calendar() {
    let connection = &mut db_connection();
    clean_database(connection);
    let result_calendar = setup_data(connection);

    let new_calendar = NewCalendar { name: "Calendar 2 updated", code: "test_calendar2" };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.put(format!("/api/calendars/{}", result_calendar.id ))
        .header(ContentType::JSON)
        .body(json::to_string(&new_calendar).unwrap())
        .dispatch();

    let result = calendars::dsl::calendars
        .find(result_calendar.id)
        .select(Calendar::as_select())
        .load(connection)
        .expect("Error loading calendars");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(result.len(), 1); // Expecting three calendars in the response
    assert_eq!(result[0].name, "Calendar 2 updated"); // Expecting three calendars in the response
    assert_eq!(result[0].code, "test_calendar2"); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_delete_calendar() {
    let conn = &mut db_connection();
    clean_database(conn);
    let result_calendar = setup_data(conn);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.delete(format!("/api/calendars/{}", result_calendar.id ))
        .header(ContentType::JSON)
        .dispatch();

    let result = calendars::dsl::calendars
        .find(result_calendar.id)
        .select(Calendar::as_select())
        .load(conn)
        .expect("Error loading calendars");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(result.len(), 0); // Expecting three calendars in the response

    clean_database(conn);
}


