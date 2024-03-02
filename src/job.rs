#[macro_use] extern crate rocket;

use rocket::Rocket;
use rocket::Build;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
}
