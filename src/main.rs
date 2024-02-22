#[macro_use] extern crate rocket;

mod actions;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![actions::hello])
}

#[cfg(test)]
mod test;
