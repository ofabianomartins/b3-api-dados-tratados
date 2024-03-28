use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;

use crate::models::Currency;
use crate::models::NewCurrency;
use crate::schema::currencies::dsl::*;
use crate::connections::db_connection;

use crate::test::clean_database;

fn setup_data(conn: &mut PgConnection) -> Currency {
    let currency = NewCurrency { name: "Calendar 2", code: "test_calendar2" };
    return insert_into(currencies)
        .values(&currency)
        .returning(Currency::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");
}

#[test]
fn test_get_currencies() {
    let connection = &mut db_connection();

    clean_database(connection);
    setup_data(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/currencies")
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);

    let test = response.into_string().unwrap();
    let currencies_list: Vec<Currency> = json::from_str(&test).expect("Failed to read JSON");
    assert_eq!(currencies_list.len(), 1); // Expecting three calendars in the response
    
    clean_database(connection);
}

#[test]
fn test_show_currency() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);
    let result_currency = setup_data(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(format!("/api/currencies/{}", result_currency.id ))
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.len(), 2); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_post_currencies() {
    let connection = &mut db_connection();

    clean_database(connection);

    // Setup: Define the data for the new calendar
    let new_currency = NewCurrency {
        // Define the fields of the new calendar here
        name: "Test Calendar",
        code: "test_calendar",
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/currencies")
        .header(ContentType::JSON)
        .body(json::to_string(&new_currency).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Created);

    clean_database(connection);
}

#[test]
fn test_update_currency() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);

    let result_currency = setup_data(connection);
    let new_currency = NewCurrency { name: "Calendar 2 updated", code: "test_calendar2" };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.put(format!("/api/currencies/{}", result_currency.id ))
        .header(ContentType::JSON)
        .body(json::to_string(&new_currency).unwrap())
        .dispatch();

    let result = currencies
        .find(result_currency.id)
        .select(Currency::as_select())
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
fn test_delete_currency() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();
    clean_database(connection);
    let result_currency = setup_data(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.delete(format!("/api/currencies/{}", result_currency.id ))
        .header(ContentType::JSON)
        .dispatch();

    let result = currencies
        .find(result_currency.id)
        .select(Currency::as_select())
        .load(connection)
        .expect("Error loading calendars");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(result.len(), 0); // Expecting three calendars in the response

    clean_database(connection);
}


