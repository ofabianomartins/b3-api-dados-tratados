use serde::Serialize;
use serde::Deserialize;

use rocket::serde::json;

pub mod calendars;
pub mod indicators;
pub mod companies;
pub mod holidays;
pub mod currencies;
pub mod tickers;
pub mod quotes;
pub mod theory_portfolios;
pub mod theory_portfolio_transactions;
pub mod sectors;
pub mod subsectors;
pub mod segments;
pub mod main;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String
}

pub fn to_resp(message: String) -> String{
    json::to_string(&MessageResponse { message: message } ).unwrap()
}
