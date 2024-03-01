use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::delete;

use crate::models::Currency;
use crate::models::NewCurrency;
use crate::schema::currencies::dsl::*;
use crate::establish_connection;

#[test]
fn test_get_currencies() {
    let connection = &mut establish_connection();

    delete(currencies)
        .execute(connection)
        .expect("Failed to delete calendars");

    let currency = NewCurrency { name: "Calendar 2", code: "test_calendar2" };
    insert_into(currencies)
        .values(&currency)
        .returning(Currency::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

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
    
    delete(currencies)
        .execute(connection)
        .expect("Failed to delete currencies");
}

#[test]
fn test_delete_currency() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut establish_connection();

    delete(currencies)
        .execute(connection)
        .expect("Failed to delete currencies");

    let currency = NewCurrency { name: "Calendar 2", code: "test_calendar2" };
    let result_currency = insert_into(currencies)
        .values(&currency)
        .returning(Currency::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

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

    delete(currencies)
        .execute(connection)
        .expect("Failed to delete currencies");
}

#[test]
fn test_post_currencies() {
    let connection = &mut establish_connection();

    delete(currencies)
        .execute(connection)
        .expect("Failed to delete calendars");

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

    delete(currencies)
        .execute(connection)
        .expect("Failed to delete calendars");
}

