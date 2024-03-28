use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;

use crate::models::Indicator;
use crate::models::NewIndicator;
use crate::schema::indicators::dsl::*;
use crate::connections::db_connection;

use crate::test::clean_database;

fn setup_data(conn: &mut PgConnection) -> Indicator {
    let new_indicator = NewIndicator {
        name: "Calendar 2",
        indicator_type: "DEFAULT",
        symbol: "CODE", 
        description: "Description of indicators"
    };
    return insert_into(indicators)
        .values(&new_indicator)
        .returning(Indicator::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");
}

#[test]
fn test_get_indicators() {
    let connection = &mut db_connection();

    clean_database(connection);
    setup_data(connection);

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/indicators")
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let test = response.into_string().unwrap();
    let indicators_list: Vec<Indicator> = json::from_str(&test).expect("Failed to read JSON");
    assert_eq!(indicators_list.len(), 1); // Expecting three calendars in the response
    
    clean_database(connection);
}

#[test]
fn test_show_indicators() {
    let connection = &mut db_connection();

    clean_database(connection);

    let result_indicator = setup_data(connection);

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(format!("/api/indicators/{}", result_indicator.id ))
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.len(), 2); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_post_indicators() {
    let connection = &mut db_connection();

    clean_database(connection);

    let new_indicator = NewIndicator { 
        name: "Calendar 2", 
        indicator_type: "DEFAULT", 
        symbol: "CODE",
        description: "Description of indicators"
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/indicators")
        .header(ContentType::JSON)
        .body(json::to_string(&new_indicator).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Created);

    clean_database(connection);
}

#[test]
fn test_delete_indicator() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);

    let result_indicator = setup_data(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.delete(format!("/api/indicators/{}", result_indicator.id ))
        .header(ContentType::JSON)
        .dispatch();

    let result = indicators
        .find(result_indicator.id)
        .select(Indicator::as_select())
            .load(connection)
        .expect("Error loading indicators");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(result.len(), 0); // Expecting three calendars in the response

    clean_database(connection);
}


