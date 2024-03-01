#[macro_use] extern crate rocket;

use rocket::Rocket;
use rocket::Build;

use diesel::prelude::*;
use std::env;

mod controllers;
mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/api", 
            routes![
                controllers::main::index,
                controllers::theory_portfolio_transactions::index,
                controllers::theory_portfolio_transactions::create,
                controllers::theory_portfolio_transactions::destroy,
                controllers::theory_portfolios::index,
                controllers::theory_portfolios::create,
                controllers::theory_portfolios::destroy,
                controllers::quotes::index,
                controllers::quotes::create,
                controllers::quotes::destroy,
                controllers::tickers::index,
                controllers::tickers::create,
                controllers::tickers::destroy,
                controllers::calendars::index,
                controllers::calendars::create,
                controllers::calendars::destroy,
                controllers::holidays::index,
                controllers::holidays::create,
                controllers::holidays::destroy,
                controllers::currencies::index,
                controllers::currencies::create,
                controllers::currencies::destroy
            ]
        )
}

#[cfg(test)]
mod test;
