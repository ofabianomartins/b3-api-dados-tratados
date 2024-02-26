use crate::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn index_should_return_string_hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}
