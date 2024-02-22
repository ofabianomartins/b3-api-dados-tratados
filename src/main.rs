#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world2!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
