#[macro_use] extern crate rocket;

use rocket::Rocket;
use rocket::Build;



mod controllers;
mod models;
mod schema;
mod connections;


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
