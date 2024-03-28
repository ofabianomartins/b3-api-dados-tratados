use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;

use crate::models::Sector;
use crate::models::NewSector;
use crate::schema::sectors::dsl::*;
use crate::connections::db_connection;

use crate::test::clean_database;

#[test]
fn test_get_sectors() {
    let connection = &mut db_connection();

    clean_database(connection);

    let sector = NewSector { name: "Calendar 2" };
    insert_into(sectors)
        .values(&sector)
        .returning(Sector::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/sectors")
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);

    let test = response.into_string().unwrap();
    let sectors_list: Vec<Sector> = json::from_str(&test).expect("Failed to read JSON");
    assert_eq!(sectors_list.len(), 1); // Expecting three calendars in the response
    
    clean_database(connection);
}

#[test]
fn test_show_sector() {
    let connection = &mut db_connection();

    clean_database(connection);

    let sector = NewSector { name: "Calendar 2" };
    let result_sector = insert_into(sectors)
        .values(&sector)
        .returning(Sector::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(format!("/api/sectors/{}", result_sector.id ))
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.len(), 2); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_post_sectors() {
    let connection = &mut db_connection();

    clean_database(connection);

    // Setup: Define the data for the new calendar
    let new_sector = NewSector { name: "Test Calendar" };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/sectors")
        .header(ContentType::JSON)
        .body(json::to_string(&new_sector).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Created);

    clean_database(connection);
}

#[test]
fn test_update_sector() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);

    let sector = NewSector { name: "Calendar 2" };
    let result_sector = insert_into(sectors)
        .values(&sector)
        .returning(Sector::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    let new_sector = NewSector { name: "Calendar 2 updated" };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.put(format!("/api/sectors/{}", result_sector.id ))
        .header(ContentType::JSON)
        .body(json::to_string(&new_sector).unwrap())
        .dispatch();

    let result = sectors
        .find(result_sector.id)
        .select(Sector::as_select())
        .load(connection)
        .expect("Error loading calendars");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(result.len(), 1); // Expecting three calendars in the response
    assert_eq!(result[0].name, "Calendar 2 updated"); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_delete_sector() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);

    let sector = NewSector { name: "Calendar 2" };
    let result_sector = insert_into(sectors)
        .values(&sector)
        .returning(Sector::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.delete(format!("/api/sectors/{}", result_sector.id ))
        .header(ContentType::JSON)
        .dispatch();

    let result = sectors
        .find(result_sector.id)
        .select(Sector::as_select())
        .load(connection)
        .expect("Error loading calendars");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(result.len(), 0); // Expecting three calendars in the response

    clean_database(connection);
}

