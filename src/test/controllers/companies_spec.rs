use crate::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;

use diesel::prelude::*;
use diesel::insert_into;

use crate::models::Company;
use crate::models::NewCompany;
use crate::schema::companies::dsl::*;
use crate::connections::db_connection;

use crate::test::clean_database;

fn setup_data(conn: &mut PgConnection) -> Company {
    let new_company = NewCompany { name: "Calendar 2", company_type: "DEFAULT", cnpj: "00.000.000/0001-00" };
    return insert_into(companies)
        .values(&new_company)
        .returning(Company::as_returning())
        .get_result(conn)
        .expect("Failed to insert sample data into the database");
}

#[test]
fn test_get_companies() {
    let connection = &mut db_connection();

    clean_database(connection);
    setup_data(connection);

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/companies")
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let test = response.into_string().unwrap();
    let companies_list: Vec<Company> = json::from_str(&test).expect("Failed to read JSON");
    assert_eq!(companies_list.len(), 1); // Expecting three calendars in the response
    
    clean_database(connection);
}

#[test]
fn test_show_companies() {
    let connection = &mut db_connection();

    clean_database(connection);

    let result_company = setup_data(connection);

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(format!("/api/companies/{}", result_company.id ))
        .header(ContentType::JSON)
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.len(), 2); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_post_companies() {
    let connection = &mut db_connection();

    clean_database(connection);

    let new_company = NewCompany { 
        name: "Calendar 2", 
        company_type: "DEFAULT", 
        cnpj: "00.000.000/0001-00" 
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/api/companies")
        .header(ContentType::JSON)
        .body(json::to_string(&new_company).unwrap())
        .dispatch();

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Created);

    clean_database(connection);
}

#[test]
fn test_update_currency() {
    let connection = &mut db_connection();

    clean_database(connection);
    let result_company = setup_data(connection);

    let new_company = NewCompany {
        name: "Company 2",
        company_type: "DEFAULT",
        cnpj: "00.001.000/0001-00"
    };

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.put(format!("/api/companies/{}", result_company.id ))
        .header(ContentType::JSON)
        .body(json::to_string(&new_company).unwrap())
        .dispatch();

    let result = companies
        .find(result_company.id)
        .select(Company::as_select())
        .load(connection)
        .expect("Error loading companies");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(result.len(), 1); // Expecting three calendars in the response
    assert_eq!(result[0].name, "Company 2"); // Expecting three calendars in the response
    // assert_eq!(result[0].cnpj, "00.001.000/0001-00"); // Expecting three calendars in the response

    clean_database(connection);
}

#[test]
fn test_delete_company() {
    // Setup: Insert sample data into the test database
    
    let connection = &mut db_connection();

    clean_database(connection);

    let result_company = setup_data(connection);

    // Action: Make a request to the route
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.delete(format!("/api/companies/{}", result_company.id ))
        .header(ContentType::JSON)
        .dispatch();

    let result = companies
        .find(result_company.id)
        .select(Company::as_select())
        .load(connection)
        .expect("Error loading companies");

    // Assert: Check if the response contains the expected data
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(result.len(), 0); // Expecting three calendars in the response

    clean_database(connection);
}


