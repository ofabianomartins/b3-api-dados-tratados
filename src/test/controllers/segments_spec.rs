use crate::rocket;

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
use crate::schema::subsectors::dsl::*;
use crate::schema::sectors::dsl::*;
use crate::schema::segments::dsl::*;
use crate::connections::db_connection;

use crate::test::clean_database;

fn setup_data(conn: &mut PgConnection) -> Segment {
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
    return insert_into(segments)
        .values(&new_segment)
        .returning(Segment::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");
}

#[test]
fn test_get_subsectors() {
    let connection = &mut db_connection();

    clean_database(connection);
    setup_data(connection);

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/subsectors")
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let test = response.into_string().unwrap();
    let subsectors_list: Vec<Subsector> = json::from_str(&test).expect("Failed to read JSON");
    assert_eq!(subsectors_list.len(), 1); // Expecting three calendars in the response
    
    clean_database(connection);
}

#[test]
fn test_delete_subsector() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);

    let result_segment = setup_data(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.delete(format!("/api/segments/{}", result_segment.id ))
        .header(ContentType::JSON)
        .dispatch();

    let result = segments
        .find(result_segment.id)
        .select(Segment::as_select())
        .load(connection)
        .expect("Error loading segments");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(result.len(), 0); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_post_subsectors() {
    let connection = &mut db_connection();

    clean_database(connection);

    let new_sector = NewSector { name: "Calendar 2" };
    let sector = insert_into(sectors)
        .values(&new_sector)
        .returning(Sector::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    let new_subsector = NewSubsector { 
        name: "Calendar 2", 
        sector_id: sector.id,
    };
    let subsector = insert_into(subsectors)
        .values(&new_subsector)
        .returning(Subsector::as_returning())
        .get_result(connection)
        .expect("Failed to insert sample data into the database");

    let new_segment = NewSegment { name: "Calendar 2", subsector_id: subsector.id };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/segments")
        .header(ContentType::JSON)
        .body(json::to_string(&new_segment).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Created);

    clean_database(connection);
}
