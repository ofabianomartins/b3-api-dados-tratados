#[macro_use] extern crate rocket;

use rocket::Rocket;
use rocket::Build;

mod controllers;
mod models;
mod schema;
mod connections;
mod services;
mod utils;
mod cors;

use crate::cors::Cors;
use crate::cors::all_options;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Cors)
        .mount("/api", 
            routes![
                all_options,
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
                controllers::calendars::show,
                controllers::calendars::create,
                controllers::calendars::update_action,
                controllers::calendars::destroy,
                controllers::holidays::index,
                controllers::holidays::show,
                controllers::holidays::create,
                controllers::holidays::destroy,
                controllers::currencies::index,
                controllers::currencies::show,
                controllers::currencies::create,
                controllers::currencies::update_action,
                controllers::currencies::destroy,
                controllers::companies::index,
                controllers::companies::show,
                controllers::companies::create,
                controllers::companies::update_action,
                controllers::companies::destroy,
                controllers::indicators::index,
                controllers::indicators::show,
                controllers::indicators::create,
                controllers::indicators::destroy,
                controllers::sectors::index,
                controllers::sectors::show,
                controllers::sectors::create,
                controllers::sectors::update_action,
                controllers::sectors::destroy,
                controllers::subsectors::index,
                controllers::subsectors::show,
                controllers::subsectors::create,
                controllers::subsectors::destroy,
                controllers::segments::index,
                controllers::segments::show,
                controllers::segments::create,
                controllers::segments::destroy
            ]
        )
}

#[cfg(test)]
mod test;
